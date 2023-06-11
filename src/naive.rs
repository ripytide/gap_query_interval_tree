use std::collections::HashMap;
use std::hash::Hash;

use discrete_range_map::{DiscreteFinite, DiscreteRangeSet};

use crate::interface::{GapQueryIntervalTree, Interval};

pub struct NaiveGapQueryIntervalTree<I, T> {
	pub inner: HashMap<I, DiscreteRangeSet<T, Interval<T>>>,
}

impl<I, T> GapQueryIntervalTree<I, T> for NaiveGapQueryIntervalTree<I, T>
where
	I: Eq + Hash,
	T: Ord + Copy + Clone + DiscreteFinite,
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

impl<I, T> NaiveGapQueryIntervalTree<I, T>
where
	I: Eq + Hash,
	T: Ord + Copy + Clone + DiscreteFinite,
{
	pub fn get_gaps(
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
