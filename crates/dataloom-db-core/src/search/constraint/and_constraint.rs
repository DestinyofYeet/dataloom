use crate::search::constraint::{OtherConstraint, SearchConstraint};

impl SearchConstraint {
    /// The current and the new constraint must match.
    pub fn and(mut self, constraint: SearchConstraint) -> Self {
        self.other = Some(Box::new(OtherConstraint::And(constraint)));
        self
    }
}
