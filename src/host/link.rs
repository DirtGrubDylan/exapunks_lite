use std::cell::RefCell;
use std::rc::{Rc, Weak};

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
    /// Creates a new link with given gate ids and host references.
    pub fn new(
        lhs_id: &str,
        lhs_host: &Rc<RefCell<Host>>,
        rhs_id: &str,
        rhs_host: &Rc<RefCell<Host>>,
    ) -> Self {
        Link {
            lhs_id: lhs_id.to_string(),
            lhs_host: Rc::downgrade(lhs_host),
            rhs_id: rhs_id.to_string(),
            rhs_host: Rc::downgrade(rhs_host),
            occupied: false,
        }
    }

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
