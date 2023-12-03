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
use core::fmt::Debug;
use discrete_range_map::discrete_range_map::{PointType, RangeType};

use discrete_range_map::{DiscreteFinite, InclusiveInterval, InclusiveRange};

use crate::interface::GapQueryIntervalTree;
use crate::naive::NaiveGapQueryIntervalTree;
use crate::no_gaps_ref::NoGapsRefGapQueryIntervalTree;
use crate::IdType;

#[derive(Clone)]
pub struct EqualityTestGapQueryIntervalTree<I, K, D> {
    naive: NaiveGapQueryIntervalTree<I, K, D>,
    no_gaps_ref: NoGapsRefGapQueryIntervalTree<I, K, D>,
}

impl<I, K, D> EqualityTestGapQueryIntervalTree<I, K, D>
where
    D: Eq + Ord + Clone + Copy + Debug,
    K: Clone + Copy + PartialEq + Debug + From<InclusiveInterval<I>> + InclusiveRange<I>,
    I: Clone + Copy + PartialEq + Debug + Ord + DiscreteFinite,
{
    fn assert_eq(&self) {
        assert_eq!(self.naive, self.no_gaps_ref.clone().into_naive());
    }
}

impl<I, K, D> GapQueryIntervalTree<I, K, D> for EqualityTestGapQueryIntervalTree<I, K, D>
where
    I: PointType + Debug,
    K: RangeType<I> + Debug + PartialEq,
    D: IdType + Debug,
{
    fn gap_query<Q>(&self, with_identifier: Option<D>, interval: Q) -> Vec<K>
    where
        Q: RangeType<I>,
    {
        let result1 = self.naive.gap_query(with_identifier, interval);
        let result2 = self.no_gaps_ref.gap_query(with_identifier, interval);

        assert_eq!(result1, result2);

        result1
    }

    fn insert(&mut self, identifiers: BTreeSet<D>, interval: K) {
        self.naive.insert(identifiers.clone(), interval);
        self.no_gaps_ref.insert(identifiers, interval);

        self.assert_eq();
    }

    fn cut<Q>(&mut self, with_identifiers: Option<BTreeSet<D>>, interval: Q)
    where
        Q: RangeType<I>,
    {
        self.naive.cut(with_identifiers.clone(), interval);
        self.no_gaps_ref.cut(with_identifiers, interval);

        self.assert_eq();
    }

    fn append(&mut self, other: &mut Self) {
        self.naive.append(&mut other.naive);
        self.no_gaps_ref.append(&mut other.no_gaps_ref);

        self.assert_eq();
    }

    fn identifiers_at_point(&self, at_point: I) -> BTreeSet<D> {
        let result1 = self.naive.identifiers_at_point(at_point);
        let result2 = self.no_gaps_ref.identifiers_at_point(at_point);

        assert_eq!(result1, result2);

        result1
    }
}
