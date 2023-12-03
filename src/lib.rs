/*
   Copyright 2023 James Forster

   This file is part of gap_query_interval_tree.

   gap_query_interval_tree is free software: you can redistribute it
   and/or modify it under the terms of the GNU Affero General Public
   License as published by the Free Software Foundation, either
   version 3 of the License, or (at your option) any later version.

   gap_query_interval_tree is distributed in the hope that it will be
   useful, but WITHOUT ANY WARRANTY; without even the implied warranty
   of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
   Affero General Public License for more details.

   You should have received a copy of the GNU Affero General Public
   License along with gap_query_interval_tree. If not, see
   <https://www.gnu.org/licenses/>.
*/

//! A crate that provides a gap-query optimized interval-tree
//! data-structure.
//!
//! `no_std` is supported and should work with the default features.
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
#![feature(btree_extract_if)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod equality_test;
pub mod interface;
pub mod naive;
pub mod no_gaps_ref;

pub use equality_test::EqualityTestGapQueryIntervalTree;
pub use interface::GapQueryIntervalTree;
pub use naive::NaiveGapQueryIntervalTree;
pub use no_gaps_ref::IdType;
pub use no_gaps_ref::NoGapsRefGapQueryIntervalTree;
