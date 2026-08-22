use crate::core::{
    column::ColumnValue,
    search::{
        constraint::{OtherConstraint, SearchConstraint},
        search_op::SearchOp,
    },
};

impl SearchConstraint {
    #[inline]
    pub fn get(self) -> (String, SearchOp, ColumnValue, Option<Box<OtherConstraint>>) {
        (self.column, self.operator, self.value, self.other)
    }
}
