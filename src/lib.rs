#![feature(let_chains)]
pub mod interface;
pub mod equality_test;
pub mod naive;
pub mod no_gaps_ref;

pub use interface::GapQueryIntervalTree;
pub use equality_test::EqualityTestGapQueryIntervalTree;
pub use naive::NaiveGapQueryIntervalTree;
pub use no_gaps_ref::NoGapsRefGapQueryIntervalTree;
