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
