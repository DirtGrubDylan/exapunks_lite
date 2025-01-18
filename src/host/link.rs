use std::cell::RefCell;
use std::rc::Weak;

use super::Host;

/// A link has two gates, identified by id's, that point to two [`Host`]s
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Link {
    lhs_id: String,
    lhs_host: Weak<RefCell<Host>>,
    rhs_id: String,
    rhs_host: Weak<RefCell<Host>>,
    occupied: bool,
}
