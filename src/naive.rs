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

use std::collections::HashMap;
use std::hash::Hash;

use discrete_range_map::{DiscreteFinite, DiscreteRangeSet, Interval};

use crate::interface::GapQueryIntervalTree;

#[derive(Debug, Clone)]
pub struct NaiveGapQueryIntervalTree<I, T> {
	pub(crate) inner: HashMap<I, DiscreteRangeSet<T, Interval<T>>>,
}

impl<I, T> PartialEq for NaiveGapQueryIntervalTree<I, T>
where
	I: Eq + Hash,
	T: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.inner == other.inner
	}
}

impl<I, T> GapQueryIntervalTree<I, T> for NaiveGapQueryIntervalTree<I, T>
where
	I: Eq + Hash,
	T: Ord + Copy + DiscreteFinite,
{
	fn gap_query(
		&self,
		with_identifier: Option<I>,
		interval: Interval<T>,
	) -> Vec<Interval<T>> {
		let gaps = self.get_gaps(with_identifier);

		gaps.overlapping(interval).copied().collect()
	}

	fn insert(&mut self, identifier: I, interval: Interval<T>) {
		self.inner
			.entry(identifier)
			.or_default()
			.insert_merge_touching_or_overlapping(interval);
	}
	fn remove(&mut self, identifier: I, interval: Interval<T>) {
		if let Some(set) = self.inner.get_mut(&identifier) {
			let _ = set.cut(interval);
		}
	}
}

impl<I, T> NaiveGapQueryIntervalTree<I, T> {
	pub fn new() -> Self {
		Self {
			inner: HashMap::new(),
		}
	}
}

impl<I, T> NaiveGapQueryIntervalTree<I, T>
where
	I: Eq + Hash,
	T: Ord + Copy + DiscreteFinite,
{
	fn get_gaps(
		&self,
		with_identifier: Option<I>,
	) -> DiscreteRangeSet<T, Interval<T>> {
		let mut total_intervals = DiscreteRangeSet::new();
		for other_identifier_intervals in
			self.inner
				.iter()
				.filter_map(|(other_identifier, intervals)| {
					if let Some(identifier) = with_identifier.as_ref() && identifier == other_identifier {
                    None
                } else {
                    Some(intervals)
                }
				}) {
			for interval in other_identifier_intervals.iter() {
				total_intervals.insert_merge_touching_or_overlapping(*interval);
			}
		}

		let gaps = total_intervals.gaps(Interval {
			start: T::MIN,
			end: T::MAX,
		});

		let mut set = DiscreteRangeSet::new();
		for gap in gaps {
			set.insert_strict(gap).unwrap();
		}

		return set;
	}
}
