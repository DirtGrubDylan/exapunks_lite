pub mod link;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

use crate::exa::Exa;
use crate::file::File;
use crate::register::basic::BasicRegister;
use crate::register::hardware::HardwareRegister;

use link::Link;

/// A Host is a sized collection to hold Files, Exas, Hardware Registers, and Links.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Host {
    id: String,
    occupancy_limit: usize,
    local_m_register: Rc<RefCell<BasicRegister>>,
    links: HashMap<String, Weak<RefCell<Link>>>,
    pending_files: HashMap<String, File>,
    files: HashMap<String, File>,
    hardware_registers: HashMap<String, HardwareRegister>,
    system_exas: HashMap<String, Exa>,
    occupying_exa_ids: HashSet<String>,
}
