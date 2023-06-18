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
use std::hash::Hash;

use discrete_range_map::{DiscreteFinite, DiscreteRangeMap, InclusiveInterval, InclusiveRange};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::interface::GapQueryIntervalTree;
use crate::naive::NaiveGapQueryIntervalTree;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoGapsRefGapQueryIntervalTree<I, K, D> {
    #[serde(bound(
        deserialize = "I: Ord + Copy + DiscreteFinite, K: InclusiveRange<I> + Copy + From<InclusiveInterval<I>> + Deserialize<'de>, D: Deserialize<'de> + Eq + Hash, "
    ))]
    inner: DiscreteRangeMap<I, K, HashSet<D>>,
}

impl<I, K, D> GapQueryIntervalTree<I, K, D> for NoGapsRefGapQueryIntervalTree<I, K, D>
where
    I: Ord + Copy + DiscreteFinite,
    K: InclusiveRange<I> + Copy + From<InclusiveInterval<I>>,
    D: Eq + Hash + Copy,
{
    fn gap_query<Q>(&self, with_identifier: Option<D>, interval: Q) -> Vec<K>
    where
        Q: InclusiveRange<I> + Copy,
    {
        match with_identifier {
            Some(identifier) => self.get_gaps_with_identifier(identifier, interval),
            None => self.get_gaps_no_identifier(interval),
        }
    }

    fn cut(&mut self, identifiers: HashSet<D>, interval: K) {
        for (cut_interval, mut cut_identifiers) in self
            .inner
            .cut(interval)
            //to soothe the borrow checker
            .collect::<Vec<_>>()
        {
            cut_identifiers.retain(|i| !identifiers.contains(i));
            self.inner
                .insert_merge_touching_if_values_equal(cut_interval, cut_identifiers)
                .unwrap();
        }
    }

    fn insert(&mut self, identifiers: HashSet<D>, interval: K) {
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
            cut_identifiers.extend(identifiers.clone());
            (cut_interval, cut_identifiers)
        });

        for (extended_interval, extended_identifiers) in extended_cut {
            self.inner
                .insert_merge_touching_if_values_equal(extended_interval, extended_identifiers)
                .unwrap();
        }
    }

    fn append(&mut self, other: &mut Self) {
        for (interval, identifiers) in other.inner.remove_overlapping(InclusiveInterval {
            start: I::MIN,
            end: I::MAX,
        }) {
            self.insert(identifiers, interval);
        }
    }
}

impl<I, K, D> NoGapsRefGapQueryIntervalTree<I, K, D>
where
    I: Ord + Copy + DiscreteFinite,
    K: InclusiveRange<I> + Copy + From<InclusiveInterval<I>>,
    D: Eq + Hash + Copy,
{
    fn get_gaps_with_identifier<Q>(&self, identifier: D, interval: Q) -> Vec<K>
    where
        Q: InclusiveRange<I> + Copy,
    {
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
        let non_end_gaps = valid_gaps
            .filter(|gap| !gap.contains(&interval.start()) && !gap.contains(&interval.end()));

        //instead of using possibly-partial end gaps we will
        //replace them with completely_iterated gaps
        //expanded on both sides outwardly only not inwardly
        let mut left_gap = self.expand_gaps_at_point_left(identifier, interval.start());
        let mut right_gap = self.expand_gaps_at_point_right(identifier, interval.end());
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
    fn get_gaps_no_identifier<Q>(&self, interval: Q) -> Vec<K>
    where
        Q: InclusiveRange<I> + Copy,
    {
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

impl<I, K, D> NoGapsRefGapQueryIntervalTree<I, K, D>
where
    I: Ord + Copy + DiscreteFinite,
    K: InclusiveRange<I> + Copy + From<InclusiveInterval<I>>,
    D: Eq + Hash + Copy,
{
    fn expand_gaps_at_point_right(&self, identifier: D, point: I) -> Option<K> {
        let overlapping_right = self.inner.overlapping(InclusiveInterval {
            start: point,
            end: I::MAX,
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
    fn expand_gaps_at_point_left(&self, identifier: D, point: I) -> Option<K> {
        //we are going in reverse since we are going left
        let overlapping_left = self
            .inner
            .overlapping(InclusiveInterval {
                start: I::MIN,
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

    pub(crate) fn into_naive(self) -> NaiveGapQueryIntervalTree<I, K, D> {
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
}

fn valid_identifier<I>(with_identifier: Option<I>, other_identifiers: &HashSet<I>) -> bool
where
    I: Eq + Hash,
{
    match with_identifier {
        Some(identifier) => {
            other_identifiers.is_empty()
                || (other_identifiers.len() == 1 && other_identifiers.contains(&identifier))
        }
        None => other_identifiers.is_empty(),
    }
}

impl<I, K, D> PartialEq for NoGapsRefGapQueryIntervalTree<I, K, D>
where
    I: PartialEq,
    K: PartialEq,
    HashSet<D>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<I, K, D> Default for NoGapsRefGapQueryIntervalTree<I, K, D>
where
    I: DiscreteFinite + Copy + Ord,
    K: InclusiveRange<I> + From<InclusiveInterval<I>> + Copy,
{
    fn default() -> Self {
        let mut map = DiscreteRangeMap::new();
        map.insert_strict(
            K::from(InclusiveInterval {
                start: I::MIN,
                end: I::MAX,
            }),
            HashSet::new(),
        )
        .unwrap();
        Self { inner: map }
    }
}
