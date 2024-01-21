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

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;

use nodit::interval::uu;
use nodit::{IntervalType, PointType};
use nodit::NoditSet;

use crate::{interface::GapQueryIntervalTree, IdType};

#[derive(Debug, Clone)]
pub struct NaiveGapQueryIntervalTree<I, K, D> {
    pub(crate) inner: BTreeMap<D, NoditSet<I, K>>,
}

impl<I, K, D> PartialEq for NaiveGapQueryIntervalTree<I, K, D>
where
    I: PartialEq,
    K: PartialEq,
    D: IdType,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<I, K, D> GapQueryIntervalTree<I, K, D> for NaiveGapQueryIntervalTree<I, K, D>
where
    I: PointType,
    K: IntervalType<I>,
    D: IdType,
{
    fn gap_query<Q>(&self, with_identifier: Option<D>, interval: Q) -> Vec<K>
    where
        Q: IntervalType<I>,
    {
        let gaps = self.get_gaps(with_identifier);

        gaps.overlapping(interval).copied().collect()
    }

    fn insert(&mut self, identifiers: BTreeSet<D>, interval: K) {
        for identifier in identifiers {
            self.inner
                .entry(identifier)
                .or_default()
                .insert_merge_touching_or_overlapping(interval);
        }
    }
    fn cut<Q>(&mut self, with_identifiers: Option<BTreeSet<D>>, interval: Q)
    where
        Q: IntervalType<I>,
    {
        match with_identifiers {
            Some(identifiers) => {
                for identifier in identifiers {
                    if let Some(set) = self.inner.get_mut(&identifier) {
                        let _ = set.cut(interval);
                    }
                }
            }
            None => {
                for set in self.inner.values_mut() {
                    let _ = set.cut(interval);
                }
            }
        }
    }

    fn append(&mut self, other: &mut Self) {
        for (identifier, intervals) in other.inner.extract_if(|_, _| true) {
            if !intervals.is_empty() {
                let store = self.inner.entry(identifier).or_default();
                for interval in intervals {
                    store.insert_merge_touching_or_overlapping(interval);
                }
            }
        }
    }

    fn identifiers_at_point(&self, at_point: I) -> BTreeSet<D> {
        self.inner
            .iter()
            .filter_map(|(identifier, intervals)| {
                if intervals.contains_point(at_point) {
                    Some(identifier)
                } else {
                    None
                }
            })
            .copied()
            .collect()
    }
}

impl<I, K, D> Default for NaiveGapQueryIntervalTree<I, K, D> {
    fn default() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }
}

impl<I, K, D> NaiveGapQueryIntervalTree<I, K, D> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<I, K, D> NaiveGapQueryIntervalTree<I, K, D>
where
    I: PointType,
    K: IntervalType<I>,
    D: IdType,
{
    fn get_gaps(&self, with_identifier: Option<D>) -> NoditSet<I, K> {
        let mut total_intervals = NoditSet::new();
        for other_identifier_intervals in
            self.inner
                .iter()
                .filter_map(|(other_identifier, intervals)| {
                    if let Some(identifier) = with_identifier.as_ref()
                        && identifier == other_identifier
                    {
                        None
                    } else {
                        Some(intervals)
                    }
                })
        {
            for interval in other_identifier_intervals.iter() {
                total_intervals.insert_merge_touching_or_overlapping(*interval);
            }
        }

        let gaps = total_intervals.gaps_untrimmed(uu());

        let mut set = NoditSet::new();
        for gap in gaps {
            set.insert_strict(gap).unwrap();
        }

        set
    }
}
