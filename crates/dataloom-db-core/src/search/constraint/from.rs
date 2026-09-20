use std::marker::PhantomData;

use crate::{
    column::ColumnValue,
    search::{constraint::SearchConstraint, search_op::SearchOp},
    traits::{DatabaseStrategyError, model::Model},
};

impl<T, M> TryFrom<(&str, T, PhantomData<M>)> for SearchConstraint
where
    T: Into<ColumnValue>,
    M: Model,
{
    type Error = DatabaseStrategyError;

    fn try_from(value: (&str, T, PhantomData<M>)) -> Result<Self, Self::Error> {
        Self::new::<M>(value.0, SearchOp::EQ, (value.1).into())
    }
}

impl<T, M> TryFrom<(&str, SearchOp, T, PhantomData<M>)> for SearchConstraint
where
    T: Into<ColumnValue>,
    M: Model,
{
    type Error = DatabaseStrategyError;

    fn try_from(value: (&str, SearchOp, T, PhantomData<M>)) -> Result<Self, Self::Error> {
        Self::new::<M>(value.0, value.1, (value.2).into())
    }
}
