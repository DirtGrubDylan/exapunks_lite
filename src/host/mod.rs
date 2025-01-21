pub mod link;

use rand::prelude::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

use crate::exa::Exa;
use crate::file::File;
use crate::register::basic::BasicRegister;
use crate::register::hardware::HardwareRegister;

use link::Link;

/// Errors from host operations.
#[derive(Debug, PartialEq, Clone)]
pub enum HostError {
    LinkDoesNotExist(String),
    NoRoomForFile(File),
}

/// A Host is a sized collection to hold a local M [`BasicRegister`], [`File`]s, [`Exa`]s,
/// [`HardwareRegister`]s, and [`Link`]s.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Host {
    pub id: String,
    occupancy_limit: usize,
    local_m_register: Rc<RefCell<BasicRegister>>,
    links: HashMap<String, Weak<RefCell<Link>>>,
    pending_files: HashMap<String, File>,
    files: HashMap<String, File>,
    hardware_registers: HashMap<String, HardwareRegister>,
    system_exas: HashMap<String, Exa>,
    occupying_exa_ids: HashSet<String>,
}

impl Host {
    /// Creates a new Host with a given id and occupancy limit.
    pub fn new(id: &str, occupancy_limit: usize) -> Self {
        Host {
            id: id.to_string(),
            occupancy_limit,
            local_m_register: Rc::new(RefCell::new(BasicRegister::new("M"))),
            links: HashMap::new(),
            files: HashMap::new(),
            pending_files: HashMap::new(),
            hardware_registers: HashMap::new(),
            system_exas: HashMap::new(),
            occupying_exa_ids: HashSet::new(),
        }
    }

    /// Inserts a [`Link`] to the map of links, using the provided gate id as the key.
    pub fn insert_link(&mut self, gate_id: &str, link: &Rc<RefCell<Link>>) {
        self.links.insert(gate_id.to_string(), Rc::downgrade(link));
    }

    /// Inserts an [`File`] to the map of files, using the file's id as the key.
    ///
    /// # Panics
    ///
    /// If there is no room in the host.
    pub fn insert_file(&mut self, file: File) {
        assert!(
            self.has_available_space(),
            "There is no available space in the Host for a file."
        );

        self.files.insert(file.id.clone(), file);
    }

    /// Inserts an [`File`] to the map of pending files, using the file's id as the key.
    /// Returns the [`File`] back in a [`HostError::NoRoomForFile`] if there is no room.
    ///
    /// # Errors
    ///
    /// * `NoRoomForFile` - if there is no room in the host.
    pub fn insert_pending_file(&mut self, file: File) -> Result<(), HostError> {
        if self.has_available_space() {
            self.pending_files.insert(file.id.clone(), file);

            Ok(())
        } else {
            Err(HostError::NoRoomForFile(file))
        }
    }

    /// Inserts an [`HardwareRegister`] to the map of hardware registers, using the register's id as
    /// the key.
    ///
    /// # Panics
    ///
    /// If there is no room in the host.
    pub fn insert_hardware_register(&mut self, register: HardwareRegister) {
        assert!(
            self.has_available_space(),
            "There is no available space in the Host for a hardware register."
        );

        self.hardware_registers
            .insert(register.id.clone(), register);
    }

    /// Inserts an [`Exa`] to the map of system exas, using the exa's id as the key.
    ///
    /// # Panics
    ///
    /// If there is no room in the host.
    pub fn insert_system_exa(&mut self, exa: Exa) {
        assert!(
            self.has_available_space(),
            "There is no available space in the Host for a system exa."
        );

        self.system_exas.insert(exa.id.clone(), exa);
    }

    /// Inserts an [`Exa`] id to the list of occupied ids.
    ///
    /// # Panics
    ///
    /// If there is no room in the host.
    pub fn insert_exa_id(&mut self, exa_id: &str) {
        assert!(
            self.has_available_space(),
            "There is no available space in the Host for an exa."
        );

        self.occupying_exa_ids.insert(exa_id.to_string());
    }

    /// Removes, and returns, a [`File`] from the list of files with a given file id, if possible.
    ///
    /// This will return [`Option::None`] if:
    /// * The [`File`] does not exist for the given file id.
    /// * The [`File`] is pending.
    pub fn remove_file(&mut self, file_id: &str) -> Option<File> {
        self.files
            .remove(file_id)
            .filter(|file| !self.pending_files.contains_key(&file.id))
    }

    /// Removes, and returns, a given id from the list of occupying exa ids.
    pub fn remove_occupying_exa_id(&mut self, exa_id: &str) -> Option<String> {
        if self.occupying_exa_ids.remove(exa_id) {
            Some(exa_id.to_string())
        } else {
            None
        }
    }

    /// Removes, and returns, a random id from the list of occupying exa ids.
    pub fn remove_random_occupying_exa_id(&mut self) -> Option<String> {
        let id = self
            .occupying_exa_ids
            .iter()
            .choose(&mut thread_rng())
            .cloned()
            .unwrap_or(String::new());

        self.remove_occupying_exa_id(&id)
    }

    /// Removes, and returns the id of, a random exa from the list of system exas.
    pub fn remove_random_system_exa(&mut self) -> Option<String> {
        let possible_id = self.system_exas.keys().choose(&mut thread_rng()).cloned();

        if let Some(id) = &possible_id {
            self.system_exas.remove(id);
        }

        possible_id
    }

    /// Returns a mutable reference to a hardware register with the given id if possible.
    pub fn hardware_register_mut(&mut self, register_id: &str) -> Option<&mut HardwareRegister> {
        self.hardware_registers.get_mut(register_id)
    }

    /// Moves all pending files to the map of files, leaving the pending files map empty.
    pub fn uptake_pending_files(&mut self) {
        self.files.extend(self.pending_files.drain());
    }

    /// Indicates if a [`File`] exists for a given file id (even if it's pending).
    pub fn has_file(&self, file_id: &str) -> bool {
        self.files.contains_key(file_id) || self.pending_files.contains_key(file_id)
    }

    /// Indicates if a [`Link`] for a given gate id.
    pub fn has_link(&self, gate_id: &str) -> bool {
        self.links.contains_key(gate_id)
    }

    /// Returns a [`Weak`] reference to the linked host for a given gate id, if possible.
    ///
    /// This will return [`Option::None`] if:
    /// * The [`Link`] does exist, but is occupied.
    /// * The destination host has no occupancy.
    ///
    /// # Errors
    ///
    /// * `LinkDoesNotExist` - if the [`Link`] does not exist for the given gate id.
    pub fn link(&self, gate_id: &str) -> Result<Option<Weak<RefCell<Host>>>, HostError> {
        if !self.has_link(gate_id) {
            return Err(HostError::LinkDoesNotExist(gate_id.to_string()));
        }

        let available_link = self
            .links
            .get(gate_id)
            .and_then(Weak::upgrade)
            .filter(|link| !link.borrow().occupied);

        let destination_host = available_link
            .as_ref()
            .and_then(|link| link.borrow().destination(gate_id))
            .and_then(|host| host.upgrade())
            .filter(|host| host.borrow().has_available_space())
            .map(|host| Rc::downgrade(&host));

        if destination_host.is_some() {
            available_link.unwrap().borrow_mut().occupied = true;
        }

        Ok(destination_host)
    }

    /// Indicates if there is available space in the host.
    ///
    /// This is determined by the number of occupying exa ids, number of files (pending included),
    /// number of hardware registers, and system exas compared to the hosts occupancy limit.
    pub fn has_available_space(&self) -> bool {
        let remaining_space = self
            .occupancy_limit
            .saturating_sub(self.files.len())
            .saturating_sub(self.pending_files.len())
            .saturating_sub(self.hardware_registers.len())
            .saturating_sub(self.system_exas.len())
            .saturating_sub(self.occupying_exa_ids.len());

        0 < remaining_space
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_err_no_link_exists() {
        let host = Host::new("host_id", 9);

        assert!(!host.has_link("800"));
        assert_eq!(
            host.link("800").err(),
            Some(HostError::LinkDoesNotExist(String::from("800")))
        );
    }

    #[test]
    fn test_link_ok_none_link_is_occupied() {
        let host_1 = Rc::new(RefCell::new(Host::new("host_1", 9)));
        let host_2 = Rc::new(RefCell::new(Host::new("host_2", 9)));

        let link = Rc::new(RefCell::new(Link::new("800", &host_2, "-1", &host_1)));

        link.borrow_mut().occupied = true;

        host_1.borrow_mut().insert_link("800", &link);
        host_2.borrow_mut().insert_link("-1", &link);

        assert!(host_1.borrow().has_link("800"));
        assert!(host_2.borrow().has_link("-1"));
        assert!(host_1.borrow_mut().link("800").unwrap().is_none());
        assert!(host_2.borrow_mut().link("-1").unwrap().is_none());
    }

    #[test]
    fn test_link_ok_none_link_destination_host_is_full() {
        let host_1 = Rc::new(RefCell::new(Host::new("host_1", 9)));
        let host_2 = Rc::new(RefCell::new(Host::new("host_2", 0)));

        let link = Rc::new(RefCell::new(Link::new("800", &host_2, "-1", &host_1)));

        host_1.borrow_mut().insert_link("800", &link);
        host_2.borrow_mut().insert_link("-1", &link);

        assert!(host_1.borrow().has_link("800"));
        assert!(host_2.borrow().has_link("-1"));
        assert!(host_1.borrow_mut().link("800").unwrap().is_none());
        assert!(!link.borrow().occupied);
        assert!(host_2.borrow_mut().link("-1").unwrap().is_some());
        assert!(link.borrow().occupied);
    }

    #[test]
    fn test_link_some_and_link_is_occupied() {
        let host_1 = Rc::new(RefCell::new(Host::new("host_1", 9)));
        let host_2 = Rc::new(RefCell::new(Host::new("host_2", 9)));

        let link = Rc::new(RefCell::new(Link::new("800", &host_2, "-1", &host_1)));

        assert!(!link.borrow().occupied);

        host_1.borrow_mut().insert_link("800", &link);
        host_2.borrow_mut().insert_link("-1", &link);

        let linked_host = host_1.borrow_mut().link("800").unwrap();
        let linked_host_id = linked_host
            .and_then(|host| host.upgrade())
            .map(|host| host.borrow().id.clone());

        assert!(link.borrow().occupied);
        assert!(host_1.borrow().has_link("800"));
        assert!(host_2.borrow().has_link("-1"));
        assert!(host_2.borrow_mut().link("-1").unwrap().is_none());
        assert_eq!(linked_host_id, Some(String::from("host_2")));
    }

    #[test]
    fn test_insert_pending_file_ok() {
        let mut host = Host::new("host_1", 9);
        let file = File::new("200");

        let result = host.insert_pending_file(file.clone());

        assert!(result.is_ok());
        assert_eq!(
            host.pending_files,
            HashMap::from([(String::from("200"), file)])
        );
    }

    #[test]
    fn test_insert_pending_file() {
        let mut host = Host::new("host_1", 1);
        let file_1 = File::new("200");
        let file_2 = File::new("201");

        let result_1 = host.insert_pending_file(file_1.clone());
        let result_2 = host.insert_pending_file(file_2.clone());

        assert!(result_1.is_ok());
        assert_eq!(result_2, Err(HostError::NoRoomForFile(file_2)));
        assert_eq!(
            host.pending_files,
            HashMap::from([(String::from("200"), file_1)])
        );
    }

    #[test]
    fn test_uptake_pending_files() {
        let mut host = Host::new("host_1", 2);
        let file_1 = File::new("200");
        let file_2 = File::new("201");

        host.insert_file(file_1.clone());
        let result_insert_pending = host.insert_pending_file(file_2.clone());

        assert!(result_insert_pending.is_ok());
        assert_eq!(
            host.files,
            HashMap::from([(String::from("200"), file_1.clone()),])
        );
        assert_eq!(
            host.pending_files,
            HashMap::from([(String::from("201"), file_2.clone()),])
        );

        host.uptake_pending_files();

        assert!(host.pending_files.is_empty());
        assert_eq!(
            host.files,
            HashMap::from([(String::from("200"), file_1), (String::from("201"), file_2),])
        );
    }
}
