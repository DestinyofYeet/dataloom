use crate::search::constraint::{OtherConstraint, SearchConstraint};

impl SearchConstraint {
    pub fn or(mut self, constraint: SearchConstraint) -> Self {
        self.other = Some(Box::new(OtherConstraint::Or(constraint)));
        self
    }
}
