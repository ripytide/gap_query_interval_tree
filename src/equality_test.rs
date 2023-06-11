use std::fmt::Debug;
use std::hash::Hash;

use discrete_range_map::DiscreteFinite;

use crate::interface::GapQueryIntervalTree;
use crate::naive::NaiveGapQueryIntervalTree;
use crate::no_gaps_ref::NoGapsRefGapQueryIntervalTree;

#[derive(Clone)]
pub struct EqualityTestGapQueryIntervalTree<I, T> {
	naive: NaiveGapQueryIntervalTree<I, T>,
	no_gaps_ref: NoGapsRefGapQueryIntervalTree<I, T>,
}

impl<I, T> EqualityTestGapQueryIntervalTree<I, T>
where
	I: Eq + Hash + Copy + Debug,
	T: Ord + Copy + DiscreteFinite + Debug,
{
	fn assert_eq(&self) {
		assert_eq!(self.naive, self.no_gaps_ref.clone().into_naive());
	}
}

impl<I, T> GapQueryIntervalTree<I, T> for EqualityTestGapQueryIntervalTree<I, T>
where
	I: Eq + Hash + Copy + Debug,
	T: Ord + Copy + DiscreteFinite + Debug,
{
	fn gap_query(
		&self,
		with_identifier: Option<I>,
		interval: discrete_range_map::Interval<T>,
	) -> Vec<discrete_range_map::Interval<T>> {
		let result1 = self.naive.gap_query(with_identifier, interval);
		let result2 = self.no_gaps_ref.gap_query(with_identifier, interval);

		assert_eq!(result1, result2);

		return result1;
	}

	fn insert(
		&mut self,
		identifier: I,
		interval: discrete_range_map::Interval<T>,
	) {
		self.naive.insert(identifier, interval);
		self.no_gaps_ref.insert(identifier, interval);

		self.assert_eq();
	}

	fn remove(
		&mut self,
		identifier: I,
		interval: discrete_range_map::Interval<T>,
	) {
		self.naive.remove(identifier, interval);
		self.no_gaps_ref.remove(identifier, interval);

		self.assert_eq();
	}
}
