use std::any::Any;

use enum_dispatch::enum_dispatch;
use polars_core::datatypes::DataType;
use polars_core::prelude::AnyValue;

use crate::executors::sinks::groupby::aggregates::count::CountAgg;
use crate::executors::sinks::groupby::aggregates::first::FirstAgg;
use crate::executors::sinks::groupby::aggregates::last::LastAgg;
use crate::executors::sinks::groupby::aggregates::mean::MeanAgg;
use crate::executors::sinks::groupby::aggregates::null::NullAgg;
use crate::executors::sinks::groupby::aggregates::SumAgg;
use crate::operators::IdxSize;

#[enum_dispatch(AggregateFunction)]
pub trait AggregateFn: Send + Sync {
    fn has_physical_agg(&self) -> bool {
        false
    }
    fn pre_agg(&mut self, _chunk_idx: IdxSize, item: &mut dyn ExactSizeIterator<Item = AnyValue>);

    fn pre_agg_u8(&mut self, _chunk_idx: IdxSize, _item: Option<u8>) {
        unimplemented!()
    }

    fn pre_agg_u16(&mut self, _chunk_idx: IdxSize, _item: Option<u16>) {
        unimplemented!()
    }
    fn pre_agg_u32(&mut self, _chunk_idx: IdxSize, _item: Option<u32>) {
        unimplemented!()
    }
    fn pre_agg_u64(&mut self, _chunk_idx: IdxSize, _item: Option<u64>) {
        unimplemented!()
    }
    fn pre_agg_i8(&mut self, _chunk_idx: IdxSize, _item: Option<i8>) {
        unimplemented!()
    }
    fn pre_agg_i16(&mut self, _chunk_idx: IdxSize, _item: Option<i16>) {
        unimplemented!()
    }
    fn pre_agg_i32(&mut self, _chunk_idx: IdxSize, _item: Option<i32>) {
        unimplemented!()
    }
    fn pre_agg_i64(&mut self, _chunk_idx: IdxSize, _item: Option<i64>) {
        unimplemented!()
    }
    fn pre_agg_f32(&mut self, _chunk_idx: IdxSize, _item: Option<f32>) {
        unimplemented!()
    }
    fn pre_agg_f64(&mut self, _chunk_idx: IdxSize, _item: Option<f64>) {
        unimplemented!()
    }

    fn dtype(&self) -> DataType;

    fn combine(&mut self, other: &dyn Any);

    fn finalize(&mut self) -> AnyValue<'static>;

    fn as_any(&self) -> &dyn Any;
}

// We dispatch via an enum
// as that saves an indirection
#[enum_dispatch]
pub enum AggregateFunction {
    First(FirstAgg),
    Last(LastAgg),
    Count(CountAgg),
    SumF32(SumAgg<f32>),
    SumF64(SumAgg<f64>),
    SumU32(SumAgg<u32>),
    SumU64(SumAgg<u64>),
    SumI32(SumAgg<i32>),
    SumI64(SumAgg<i64>),
    MeanF32(MeanAgg<f32>),
    MeanF64(MeanAgg<f64>),
    Null(NullAgg), // place holder for any aggregate function
                   // this is not preferred because of the extra
                   // indirection
                   // Other(Box<dyn AggregateFn>)
}

impl AggregateFunction {
    pub(crate) fn split2(&self) -> Self {
        use AggregateFunction::*;
        match self {
            First(agg) => First(FirstAgg::new(agg.dtype.clone())),
            Last(agg) => Last(LastAgg::new(agg.dtype.clone())),
            SumF32(_) => SumF32(SumAgg::new()),
            SumF64(_) => SumF64(SumAgg::new()),
            SumU32(_) => SumU32(SumAgg::new()),
            SumU64(_) => SumU64(SumAgg::new()),
            SumI32(_) => SumI32(SumAgg::new()),
            SumI64(_) => SumI64(SumAgg::new()),
            MeanF32(_) => MeanF32(MeanAgg::new()),
            MeanF64(_) => MeanF64(MeanAgg::new()),
            Count(_) => Count(CountAgg::new()),
            Null(a) => Null(a.clone()),
        }
    }
}
