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
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::RangeBounds;

use discrete_range_map::{DiscreteFinite, InclusiveInterval, InclusiveRange};

use crate::interface::GapQueryIntervalTree;
use crate::naive::NaiveGapQueryIntervalTree;
use crate::no_gaps_ref::NoGapsRefGapQueryIntervalTree;

#[derive(Clone)]
pub struct EqualityTestGapQueryIntervalTree<I, K, D> {
    naive: NaiveGapQueryIntervalTree<I, K, D>,
    no_gaps_ref: NoGapsRefGapQueryIntervalTree<I, K, D>,
}

impl<I, K, D> EqualityTestGapQueryIntervalTree<I, K, D>
where
    D: Eq + Hash + Clone + Copy + Debug,
    K: Clone
        + Copy
        + PartialEq
        + Debug
        + RangeBounds<I>
        + From<InclusiveInterval<I>>
        + InclusiveRange<I>,
    I: Clone + Copy + PartialEq + Debug + Ord + DiscreteFinite,
{
    fn assert_eq(&self) {
        assert_eq!(self.naive, self.no_gaps_ref.clone().into_naive());
    }
}

impl<I, K, D> GapQueryIntervalTree<I, K, D> for EqualityTestGapQueryIntervalTree<I, K, D>
where
    I: Ord + Copy + DiscreteFinite + Debug,
    K: InclusiveRange<I> + Copy + From<InclusiveInterval<I>> + Debug + PartialEq,
    D: Eq + Hash + Debug + Copy,
{
    fn gap_query<Q>(&self, with_identifier: Option<D>, interval: Q) -> Vec<K>
    where
        Q: InclusiveRange<I> + Copy,
    {
        let result1 = self.naive.gap_query(with_identifier, interval);
        let result2 = self.no_gaps_ref.gap_query(with_identifier, interval);

        assert_eq!(result1, result2);

        return result1;
    }

    fn insert(&mut self, identifiers: HashSet<D>, interval: K) {
        self.naive.insert(identifiers.clone(), interval);
        self.no_gaps_ref.insert(identifiers, interval);

        self.assert_eq();
    }

    fn cut(&mut self, identifiers: HashSet<D>, interval: K) {
        self.naive.cut(identifiers.clone(), interval);
        self.no_gaps_ref.cut(identifiers, interval);

        self.assert_eq();
    }

    fn append(&mut self, other: &mut Self) {
        self.naive.append(&mut other.naive);
        self.no_gaps_ref.append(&mut other.no_gaps_ref);

        self.assert_eq();
    }
}
