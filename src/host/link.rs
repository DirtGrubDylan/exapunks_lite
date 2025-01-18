use std::cell::RefCell;
use std::rc::Weak;

use super::Host;

/// A link has two gates, identified by id's, that point to two [`Host`]s
#[derive(Debug, Clone)]
pub struct Link {
    lhs_id: String,
    lhs_host: Weak<RefCell<Host>>,
    rhs_id: String,
    rhs_host: Weak<RefCell<Host>>,
    pub occupied: bool,
}

impl Link {
    /// Returns the relative [`Host`] reference for the given gate id if possible.
    #[must_use]
    pub fn destination(&self, gate_id: &str) -> Option<Weak<RefCell<Host>>> {
        if self.lhs_id == gate_id {
            Some(self.lhs_host.clone())
        } else if self.rhs_id == gate_id {
            Some(self.rhs_host.clone())
        } else {
            None
        }
    }
}
