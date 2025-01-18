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
    ///
    /// # Panics
    ///
    /// If there is no room in the host.
    pub fn insert_pending_file(&mut self, file: File) {
        assert!(
            self.has_available_space(),
            "There is no available space in the Host for a pending file."
        );

        self.pending_files.insert(file.id.clone(), file);
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

    /// Removes a given id from the list of occupying exa ids.
    pub fn remove_occupying_exa_id(&mut self, exa_id: &str) {
        self.occupying_exa_ids.remove(exa_id);
    }

    /// Removes, and returns, a random id from the list of occupying exa ids.
    pub fn remove_random_occupying_exa_id(&mut self) -> Option<String> {
        let possible_id = self
            .occupying_exa_ids
            .iter()
            .choose(&mut thread_rng())
            .cloned();

        if let Some(id) = &possible_id {
            self.occupying_exa_ids.remove(id);
        }

        possible_id
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
    /// * The [`Link`] does not exist for the given gate id.
    /// * The [`Link`] does exist, but is occupied.
    /// * The destination host has no occupancy.
    pub fn link(&self, gate_id: &str) -> Option<Weak<RefCell<Host>>> {
        self.links
            .get(gate_id)
            .and_then(Weak::upgrade)
            .filter(|link| !link.borrow().occupied)
            .and_then(|link| {
                link.borrow_mut().occupied = true;

                link.borrow().destination(gate_id)
            })
            .and_then(|host| host.upgrade())
            .filter(|host| host.borrow().has_available_space())
            .map(|host| Rc::downgrade(&host))
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
    fn test_link_none_no_link_exists() {
        let host = Host::new("host_id", 9);

        assert!(host.link("800").is_none());
    }

    #[test]
    fn test_link_none_link_is_occupied() {
        unimplemented!()
    }

    #[test]
    fn test_link_none_link_destination_host_is_full() {
        unimplemented!()
    }

    #[test]
    fn test_link_some_and_link_is_occupied() {
        let host_1 = Rc::new(RefCell::new(Host::new("host_1", 9)));
        let host_2 = Rc::new(RefCell::new(Host::new("host_2", 9)));

        let link = Rc::new(RefCell::new(Link::new("800", &host_2, "-1", &host_1)));

        assert!(!link.borrow().occupied);

        host_1.borrow_mut().insert_link("800", &link);
        host_2.borrow_mut().insert_link("-1", &link);

        let linked_host = host_1.borrow_mut().link("800");
        let linked_host_id = linked_host
            .and_then(|host| host.upgrade())
            .map(|host| host.borrow().id.clone());

        assert_eq!(linked_host_id, Some(String::from("host_2")));
        assert!(link.borrow().occupied);
    }

    #[test]
    fn test_take_file() {
        unimplemented!()
    }

    #[test]
    fn test_uptake_files() {
        unimplemented!()
    }

    #[test]
    fn test_has_available_space() {
        unimplemented!()
    }
}
