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

use std::collections::HashSet;

use discrete_range_map::{InclusiveInterval, InclusiveRange};

pub trait GapQueryIntervalTree<I, K, D> {
    //optimisation make this a iterator not a vec to save allocation
    //
    /// Gets the maximally-sized gaps that overlap the given interval
    /// for the given identifier if one is given.
    #[doc=include_str!("../images/gap-query.svg")]
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use discrete_range_map::InclusiveInterval;
    /// use gap_query_interval_tree::{
    /// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
    /// };
    ///
    /// let mut tree = NoGapsRefGapQueryIntervalTree::new();
    /// tree.insert(HashSet::from([5]), InclusiveInterval { start: 3, end: 6 });
    /// tree.insert(HashSet::from([9]), InclusiveInterval { start: 12, end: 28 });
    ///
    /// assert_eq!(
    /// 	tree.gap_query(None, InclusiveInterval { start: 9, end: 9 }),
    /// 	Vec::from([InclusiveInterval { start: 7, end: 11 }])
    /// );
    /// ```
    fn gap_query<Q>(&self, with_identifier: Option<D>, interval: Q) -> Vec<K>
    where
        Q: InclusiveRange<I> + Copy;

    /// Inserts an interval into the collection for the given
    /// identifiers.
    #[doc=include_str!("../images/insertion.svg")]
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use discrete_range_map::InclusiveInterval;
    /// use gap_query_interval_tree::{
    /// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
    /// };
    ///
    /// let mut tree = NoGapsRefGapQueryIntervalTree::new();
    /// tree.insert(HashSet::from([5]), InclusiveInterval { start: 3, end: 6 });
    /// tree.insert(HashSet::from([9]), InclusiveInterval { start: 12, end: 28 });
    /// ```
    fn insert(&mut self, identifiers: HashSet<D>, interval: K);

    /// Cuts an interval from the collection for the given
    /// identifiers.
    #[doc=include_str!("../images/removal.svg")]
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use discrete_range_map::InclusiveInterval;
    /// use gap_query_interval_tree::{
    /// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
    /// };
    ///
    /// let mut tree = NoGapsRefGapQueryIntervalTree::new();
    /// tree.insert(HashSet::from([5]), InclusiveInterval { start: 3, end: 6 });
    /// tree.insert(HashSet::from([9]), InclusiveInterval { start: 12, end: 28 });
    ///
    /// tree.cut(HashSet::from([5]), InclusiveInterval { start: 4, end: 5 });
    /// tree.cut(HashSet::from([9]), InclusiveInterval { start: 0, end: 30 });
    /// ```
    fn cut(&mut self, identifiers: HashSet<D>, interval: K);

    /// A convenience method for appending one interval tree with
    /// another.
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use discrete_range_map::InclusiveInterval;
    /// use gap_query_interval_tree::{
    /// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
    /// };
    ///
    /// let mut tree1 = NoGapsRefGapQueryIntervalTree::new();
    /// let mut tree2 = NoGapsRefGapQueryIntervalTree::new();
    ///
    /// tree1.insert(HashSet::from([5]), InclusiveInterval { start: 3, end: 6 });
    /// tree2.insert(HashSet::from([9]), InclusiveInterval { start: 12, end: 28 });
    ///
    /// tree1.append(&mut tree2);
    ///
    /// assert_eq!(
    /// 	tree1.gap_at_point(None, 9),
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
    /// use std::collections::HashSet;
    /// use discrete_range_map::InclusiveInterval;
    /// use gap_query_interval_tree::{
    /// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
    /// };
    ///
    /// let mut tree = NoGapsRefGapQueryIntervalTree::new();
    /// tree.insert(HashSet::from([5]), InclusiveInterval { start: 3, end: 6 });
    /// tree.insert(HashSet::from([9]), InclusiveInterval { start: 12, end: 28 });
    ///
    /// assert_eq!(
    /// 	tree.gap_at_point(None, 9),
    /// 	Some(InclusiveInterval { start: 7, end: 11 })
    /// );
    ///
    /// assert_eq!(
    /// 	tree.gap_at_point(None, 9),
    /// 	tree.gap_query(None, InclusiveInterval { start: 9, end: 9 }).pop()
    /// );
    /// ```
    fn gap_at_point(&self, with_identifier: Option<D>, at_point: I) -> Option<K>
    where
        I: Copy,
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
}
