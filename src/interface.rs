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

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use discrete_range_map::{
    discrete_range_map::{PointType, RangeType},
    InclusiveInterval,
};

pub trait GapQueryIntervalTree<I, K, D> {
    //optimisation make this a iterator not a vec to save allocation
    //
    /// Gets the maximally-sized gaps that overlap the given interval
    /// for the given identifier if one is given.
    #[doc=include_str!("../images/gap-query.svg")]
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use discrete_range_map::InclusiveInterval;
    /// use gap_query_interval_tree::{
    /// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
    /// };
    ///
    /// let mut tree = NoGapsRefGapQueryIntervalTree::new();
    /// tree.insert(BTreeSet::from([5]), InclusiveInterval { start: 3, end: 6 });
    /// tree.insert(BTreeSet::from([9]), InclusiveInterval { start: 12, end: 28 });
    ///
    /// assert_eq!(
    /// 	tree.gap_query(None, InclusiveInterval { start: 9, end: 9 }),
    /// 	Vec::from([InclusiveInterval { start: 7, end: 11 }])
    /// );
    /// ```
    fn gap_query<Q>(&self, with_identifier: Option<D>, interval: Q) -> Vec<K>
    where
        Q: RangeType<I>;

    /// Inserts an interval into the collection for the given
    /// identifiers.
    #[doc=include_str!("../images/insertion.svg")]
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use discrete_range_map::InclusiveInterval;
    /// use gap_query_interval_tree::{
    /// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
    /// };
    ///
    /// let mut tree = NoGapsRefGapQueryIntervalTree::new();
    /// tree.insert(BTreeSet::from([5]), InclusiveInterval { start: 3, end: 6 });
    /// tree.insert(BTreeSet::from([9]), InclusiveInterval { start: 12, end: 28 });
    /// ```
    fn insert(&mut self, identifiers: BTreeSet<D>, interval: K);

    /// Cuts an interval from the collection for the given
    /// identifiers, if no identifiers are given all identifiers are
    /// cut.
    #[doc=include_str!("../images/removal.svg")]
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use discrete_range_map::InclusiveInterval;
    /// use gap_query_interval_tree::{
    /// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
    /// };
    ///
    /// let mut tree = NoGapsRefGapQueryIntervalTree::new();
    /// tree.insert(BTreeSet::from([5]), InclusiveInterval { start: 3, end: 6 });
    /// tree.insert(BTreeSet::from([9]), InclusiveInterval { start: 12, end: 28 });
    ///
    /// tree.cut(Some(BTreeSet::from([5])), InclusiveInterval { start: 4, end: 5 });
    /// tree.cut(Some(BTreeSet::from([9])), InclusiveInterval { start: 0, end: 30 });
    /// ```
    fn cut<Q>(&mut self, with_identifiers: Option<BTreeSet<D>>, interval: Q)
    where
        Q: RangeType<I>;

    /// Append one interval tree with another by inserting all the
    /// intervals from `other` into `self`.
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use discrete_range_map::InclusiveInterval;
    /// use gap_query_interval_tree::{
    /// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
    /// };
    ///
    /// let mut tree1 = NoGapsRefGapQueryIntervalTree::new();
    /// let mut tree2 = NoGapsRefGapQueryIntervalTree::new();
    ///
    /// tree1.insert(BTreeSet::from([5]), InclusiveInterval { start: 3, end: 6 });
    /// tree2.insert(BTreeSet::from([9]), InclusiveInterval { start: 12, end: 28 });
    ///
    /// tree1.append(&mut tree2);
    ///
    /// assert_eq!(
    /// 	tree1.gap_query_at_point(None, 9),
    /// 	Some(InclusiveInterval { start: 7, end: 11 })
    /// );
    /// ```
    fn append(&mut self, other: &mut Self);

    /// A convenience method for getting the maximally-sized gap at a
    /// specific point for the given identifier if one is given, this
    /// is equivalent to calling.
    /// [`gap_query()`](GapQueryIntervalTree::gap_query) with a point
    /// interval.
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use discrete_range_map::InclusiveInterval;
    /// use gap_query_interval_tree::{
    /// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
    /// };
    ///
    /// let mut tree = NoGapsRefGapQueryIntervalTree::new();
    /// tree.insert(BTreeSet::from([5]), InclusiveInterval { start: 3, end: 6 });
    /// tree.insert(BTreeSet::from([9]), InclusiveInterval { start: 12, end: 28 });
    ///
    /// assert_eq!(
    /// 	tree.gap_query_at_point(None, 9),
    /// 	Some(InclusiveInterval { start: 7, end: 11 })
    /// );
    ///
    /// assert_eq!(
    /// 	tree.gap_query_at_point(None, 9),
    /// 	tree.gap_query(None, InclusiveInterval { start: 9, end: 9 }).pop()
    /// );
    /// ```
    fn gap_query_at_point(&self, with_identifier: Option<D>, at_point: I) -> Option<K>
    where
        I: PointType,
    {
        let mut overlapping = self.gap_query(
            with_identifier,
            InclusiveInterval {
                start: at_point,
                end: at_point,
            },
        );

        assert!(overlapping.is_empty() || overlapping.len() == 1);

        overlapping.pop()
    }

    /// Get all identifiers which have an interval overlapping the
    /// given point.
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use discrete_range_map::InclusiveInterval;
    /// use gap_query_interval_tree::{
    /// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
    /// };
    ///
    /// let mut tree = NoGapsRefGapQueryIntervalTree::new();
    /// tree.insert(BTreeSet::from([5]), InclusiveInterval { start: 3, end: 6 });
    /// tree.insert(BTreeSet::from([9]), InclusiveInterval { start: 12, end: 28 });
    ///
    /// assert_eq!(
    /// 	tree.identifiers_at_point(9),
    /// 	BTreeSet::from([])
    /// );
    ///
    /// assert_eq!(
    /// 	tree.identifiers_at_point(16),
    /// 	BTreeSet::from([9])
    /// );
    /// ```
    fn identifiers_at_point(&self, at_point: I) -> BTreeSet<D>
    where
        D: Copy;
}
