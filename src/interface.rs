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

use discrete_range_map::Interval;

/// A GapQueryIntervalTree as described in my paper
pub trait GapQueryIntervalTree<I, T>
where
	T: Copy + Clone,
{
	//optimisation make this a iterator not a vec to save allocation
	//
	/// Gets the maximally-sized gaps that overlap the given interval
	/// for the given identifier if one is given.
	#[doc=include_str!("../images/gap-query.svg")]
	///
	/// ```
	/// use discrete_range_map::Interval;
	/// use gap_query_interval_tree::{
	/// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
	/// };
	///
	/// let mut tree = NoGapsRefGapQueryIntervalTree::new();
	/// tree.insert(5, Interval { start: 3, end: 6 });
	/// tree.insert(9, Interval { start: 12, end: 28 });
	///
	/// assert_eq!(
	/// 	tree.gap_query(None, Interval { start: 9, end: 9 }),
	/// 	Vec::from([Interval { start: 7, end: 11 }])
	/// );
	/// ```
	fn gap_query(
		&self,
		with_identifier: Option<I>,
		interval: Interval<T>,
	) -> Vec<Interval<T>>;

	/// Inserts an interval into the collection for the given
	/// identifier.
	#[doc=include_str!("../images/insertion.svg")]
	///
	/// ```
	/// use discrete_range_map::Interval;
	/// use gap_query_interval_tree::{
	/// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
	/// };
	///
	/// let mut tree = NoGapsRefGapQueryIntervalTree::new();
	/// tree.insert(5, Interval { start: 3, end: 6 });
	/// tree.insert(9, Interval { start: 12, end: 28 });
	/// ```
	fn insert(&mut self, identifier: I, interval: Interval<T>);

	/// Removes an interval from the collection for the given
	/// identifier.
	#[doc=include_str!("../images/removal.svg")]
	///
	/// ```
	/// use discrete_range_map::Interval;
	/// use gap_query_interval_tree::{
	/// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
	/// };
	///
	/// let mut tree = NoGapsRefGapQueryIntervalTree::new();
	/// tree.insert(5, Interval { start: 3, end: 6 });
	/// tree.insert(9, Interval { start: 12, end: 28 });
	///
	/// tree.remove(5, Interval { start: 4, end: 5 });
	/// tree.remove(9, Interval { start: 0, end: 30 });
	/// ```
	fn remove(&mut self, identifier: I, interval: Interval<T>);

	/// A convenience method for getting the maximally-sized gap at a
	/// specific point for the given identifier if one is given, this
	/// is equivalent to calling.
	/// [`gap_query()`](GapQueryIntervalTree::gap_query) with a point
	/// interval.
	///
	/// ```
	/// use discrete_range_map::Interval;
	/// use gap_query_interval_tree::{
	/// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
	/// };
	///
	/// let mut tree = NoGapsRefGapQueryIntervalTree::new();
	/// tree.insert(5, Interval { start: 3, end: 6 });
	/// tree.insert(9, Interval { start: 12, end: 28 });
	///
	/// assert_eq!(
	/// 	tree.gap_at_point(None, 9),
	/// 	Some(Interval { start: 7, end: 11 })
	/// );
	///
	/// assert_eq!(
	/// 	tree.gap_at_point(None, 9),
	/// 	tree.gap_query(None, Interval { start: 9, end: 9 }).pop()
	/// );
	/// ```
	fn gap_at_point(
		&self,
		with_identifier: Option<I>,
		at_point: T,
	) -> Option<Interval<T>> {
		let mut overlapping = self.gap_query(
			with_identifier,
			Interval {
				start: at_point,
				end: at_point,
			},
		);

		assert!(overlapping.is_empty() || overlapping.len() == 1);

		overlapping.pop()
	}
}
