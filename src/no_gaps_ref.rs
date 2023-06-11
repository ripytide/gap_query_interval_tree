use std::collections::HashSet;
use std::hash::Hash;

use discrete_range_map::{DiscreteFinite, DiscreteRangeMap, Interval};
use itertools::Itertools;

use crate::interface::GapQueryIntervalTree;
use crate::naive::NaiveGapQueryIntervalTree;

#[derive(Clone, Debug)]
pub struct NoGapsRefGapQueryIntervalTree<I, T> {
	inner: DiscreteRangeMap<T, Interval<T>, HashSet<I>>,
}

impl<I, T> GapQueryIntervalTree<I, T> for NoGapsRefGapQueryIntervalTree<I, T>
where
	I: Eq + Hash + Copy,
	T: Ord + Copy + DiscreteFinite,
{
	fn gap_query(
		&self,
		with_identifier: Option<I>,
		interval: Interval<T>,
	) -> Vec<Interval<T>> {
		match with_identifier {
			Some(identifier) => {
				self.get_gaps_with_identifier(identifier, interval)
			}
			None => self.get_gaps_no_identifier(interval),
		}
	}

	fn remove(&mut self, identifier: I, interval: Interval<T>) {
		for (cut_interval, mut cut_identifiers) in self
			.inner
			.cut(interval)
			//to soothe the borrow checker
			.collect::<Vec<_>>()
		{
			cut_identifiers.remove(&identifier);
			self.inner
				.insert_merge_touching_if_values_equal(
					cut_interval,
					cut_identifiers,
				)
				.unwrap();
		}
	}

	fn insert(&mut self, identifier: I, interval: Interval<T>) {
		//first we extend the overlapping partial
		//intervals with the
		//other_specifiers and then insert them
		//back into the data_structure with
		//insert_merge_touching_if_values_equal to prevent
		//fragmentation
		//
		//optimisation: do this without cutting and re-inserting
		//using overlapping_mut or something
		let cut = self
			.inner
			.cut(interval)
			//to soothe the borrow checker
			.collect::<Vec<_>>()
			.into_iter();

		let extended_cut = cut.map(|(cut_interval, mut cut_identifiers)| {
			cut_identifiers.insert(identifier);
			(cut_interval, cut_identifiers)
		});

		for (extended_interval, extended_identifiers) in extended_cut {
			self.inner
				.insert_merge_touching_if_values_equal(
					extended_interval,
					extended_identifiers,
				)
				.unwrap();
		}
	}
}

impl<I, T> NoGapsRefGapQueryIntervalTree<I, T>
where
	I: Eq + Hash + Copy,
	T: Ord + Copy + DiscreteFinite,
{
	fn get_gaps_with_identifier(
		&self,
		identifier: I,
		interval: Interval<T>,
	) -> Vec<Interval<T>> {
		let valid_gaps = self
			.inner
			.overlapping(interval)
			.filter_map(move |(inner_interval, other_identifiers)| {
				if valid_identifier(Some(identifier), other_identifiers) {
					Some(inner_interval)
				} else {
					None
				}
			})
			.copied();
		//we don't want end ones as they are
		//handled separately
		let non_end_gaps = valid_gaps.filter(|gap| {
			!gap.contains(interval.start) && !gap.contains(interval.end)
		});

		//instead of using possibly-partial end gaps we will
		//replace them with completely_iterated gaps
		//expanded on both sides outwardly only not inwardly
		let mut left_gap =
			self.expand_gaps_at_point_left(identifier, interval.start);
		let mut right_gap =
			self.expand_gaps_at_point_right(identifier, interval.end);
		//if they refer to the save gap then merge them
		if let Some(left) = left_gap.as_mut()
                    && let Some(right) = right_gap
                    && left.overlaps_ordered(&right)
                {
                    *left = left.merge_ordered(&right);
                    right_gap = None;
                }

		//then we need to chain these iterators together and
		//progressively merge touching gaps
		let all_non_merged_gaps = left_gap
			.into_iter()
			.chain(non_end_gaps)
			.chain(right_gap.into_iter());

		//the final proper merged result
		all_non_merged_gaps
			.coalesce(|x, y| {
				if x.touches_ordered(&y) {
					Ok(x.merge_ordered(&y))
				} else {
					Err((x, y))
				}
			})
			.collect()
	}
	fn get_gaps_no_identifier(
		&self,
		interval: Interval<T>,
	) -> Vec<Interval<T>> {
		self.inner
			.overlapping(interval)
			.filter_map(|(inner_interval, other_specifiers)| {
				if other_specifiers.is_empty() {
					Some(inner_interval)
				} else {
					None
				}
			})
			.copied()
			.collect()
	}
}

impl<I, T> NoGapsRefGapQueryIntervalTree<I, T>
where
	I: Eq + Hash + Copy,
	T: Ord + Copy + DiscreteFinite,
{
	fn expand_gaps_at_point_right(
		&self,
		identifier: I,
		point: T,
	) -> Option<Interval<T>> {
		let overlapping_right = self.inner.overlapping(Interval {
			start: point,
			end: T::MAX,
		});

		overlapping_right
			.take_while(|(_, other_identifiers)| {
				valid_identifier(Some(identifier), other_identifiers)
			})
			.map(|(x, _)| *x)
			.coalesce(|x, y| {
				//since there are no gaps we know they will always
				//touch
				Ok(x.merge_ordered(&y))
			})
			.next()
	}
	fn expand_gaps_at_point_left(
		&self,
		identifier: I,
		point: T,
	) -> Option<Interval<T>> {
		//we are going in reverse since we are going left
		let overlapping_left = self
			.inner
			.overlapping(Interval {
				start: T::MIN,
				end: point,
			})
			.rev();

		overlapping_left
			.take_while(|(_, other_identifiers)| {
				valid_identifier(Some(identifier), other_identifiers)
			})
			.map(|(x, _)| *x)
			.coalesce(|x, y| {
				//since we are going from right to left these will
				//be reversed too
				//
				//since there are no gaps we know they will always
				//touch
				Ok(y.merge_ordered(&x))
			})
			.next()
	}

	pub(crate) fn into_naive(self) -> NaiveGapQueryIntervalTree<I, T> {
		let mut naive = NaiveGapQueryIntervalTree::new();

		for (interval, identifiers) in self.inner {
			for identifier in identifiers {
				naive
					.inner
					.entry(identifier)
					.or_default()
					.insert_merge_touching(interval)
					.unwrap();
			}
		}

		return naive;
	}

	pub fn new() -> Self {
		let mut map = DiscreteRangeMap::new();
		map.insert_strict(
			Interval {
				start: T::MIN,
				end: T::MAX,
			},
			HashSet::new(),
		)
		.unwrap();
		Self { inner: map }
	}
}

fn valid_identifier<I>(
	with_identifier: Option<I>,
	other_identifiers: &HashSet<I>,
) -> bool
where
	I: Eq + Hash,
{
	match with_identifier {
		Some(identifier) => {
			other_identifiers.is_empty()
				|| (other_identifiers.len() == 1
					&& other_identifiers.contains(&identifier))
		}
		None => other_identifiers.is_empty(),
	}
}
