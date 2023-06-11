//! A crate that provides a gap-query optimized interval-tree
//! data-structure.
//!
//! There are three main operations available on this data-structure:
//! insertion, removal and gap-queries. Each of which are `O(log(N) +
//! K)` where `N` is the total number of intervals in the tree and `K`
//! is the number of intervals required to be processed.
//!
//! Here are visualizations of the three operations:
//!
//! # Insertion
#![doc=include_str!("../images/insertion.svg")]
//! # Removal
#![doc=include_str!("../images/removal.svg")]
//! # Gap-Query
#![doc=include_str!("../images/gap-query.svg")]
#![feature(let_chains)]

pub mod equality_test;
pub mod interface;
pub mod naive;
pub mod no_gaps_ref;

pub use equality_test::EqualityTestGapQueryIntervalTree;
pub use interface::GapQueryIntervalTree;
pub use naive::NaiveGapQueryIntervalTree;
pub use no_gaps_ref::NoGapsRefGapQueryIntervalTree;
