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

use itertools::Itertools;
use nodit::interval::{iu, ui, uu};
use nodit::NoditMap;
use nodit::{IntervalType, PointType};
use serde::{Deserialize, Serialize};

use crate::interface::GapQueryIntervalTree;
use crate::naive::NaiveGapQueryIntervalTree;

pub trait IdType: Eq + Ord + Copy {}
impl<D> IdType for D where D: Eq + Ord + Copy {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoGapsRefGapQueryIntervalTree<I, K, D> {
    #[serde(bound(
        deserialize = "I: PointType, K: IntervalType<I> + Deserialize<'de>, D: IdType + Deserialize<'de>,"
    ))]
    inner: NoditMap<I, K, BTreeSet<D>>,
}

impl<I, K, D> GapQueryIntervalTree<I, K, D> for NoGapsRefGapQueryIntervalTree<I, K, D>
where
    I: PointType,
    K: IntervalType<I>,
    D: IdType,
{
    fn gap_query<Q>(&self, with_identifier: Option<D>, interval: Q) -> Vec<K>
    where
        Q: IntervalType<I>,
    {
        match with_identifier {
            Some(identifier) => self.get_gaps_with_identifier(identifier, interval),
            None => self.get_gaps_no_identifier(interval),
        }
    }

    fn cut<Q>(&mut self, with_identifiers: Option<BTreeSet<D>>, interval: Q)
    where
        Q: IntervalType<I>,
    {
        for (cut_interval, mut cut_identifiers) in self
            .inner
            .cut(interval)
            //to soothe the borrow checker
            .collect::<Vec<_>>()
        {
            match with_identifiers.as_ref() {
                Some(identifiers) => {
                    cut_identifiers.retain(|i| !identifiers.contains(i));
                }
                None => cut_identifiers.clear(),
            }
            self.inner
                .insert_merge_touching_if_values_equal(cut_interval, cut_identifiers)
                .unwrap_or_else(|_| panic!());
        }
    }

    fn insert(&mut self, identifiers: BTreeSet<D>, interval: K) {
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
                .unwrap_or_else(|_| panic!());
        }
    }

    fn append(&mut self, other: &mut Self) {
        for (interval, identifiers) in other.inner.remove_overlapping(uu()) {
            self.insert(identifiers, interval);
        }
    }

    fn identifiers_at_point(&self, at_point: I) -> BTreeSet<D> {
        self.inner
            .get_at_point(at_point)
            .cloned()
            .unwrap_or(BTreeSet::new())
    }
}

impl<I, K, D> NoGapsRefGapQueryIntervalTree<I, K, D>
where
    I: PointType,
    K: IntervalType<I>,
    D: IdType,
{
    fn get_gaps_with_identifier<Q>(&self, identifier: D, interval: Q) -> Vec<K>
    where
        Q: IntervalType<I>,
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
            .filter(|gap| !gap.contains(interval.start()) && !gap.contains(interval.end()));

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
        let all_non_merged_gaps = left_gap.into_iter().chain(non_end_gaps).chain(right_gap);

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
        Q: IntervalType<I>,
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
    I: PointType,
    K: IntervalType<I>,
    D: Eq + Ord + Copy,
{
    fn expand_gaps_at_point_right(&self, identifier: D, point: I) -> Option<K> {
        let overlapping_right = self.inner.overlapping(iu(point));

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
        let overlapping_left = self.inner.overlapping(ui(point)).rev();

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

        naive
    }
}

fn valid_identifier<I>(with_identifier: Option<I>, other_identifiers: &BTreeSet<I>) -> bool
where
    I: Eq + Ord,
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
    BTreeSet<D>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<I, K, D> Default for NoGapsRefGapQueryIntervalTree<I, K, D>
where
    I: PointType,
    K: IntervalType<I>,
{
    fn default() -> Self {
        let mut map = NoditMap::new();
        map.insert_strict(K::from(uu()), BTreeSet::new())
            .unwrap_or_else(|_| panic!());
        Self { inner: map }
    }
}

impl<I, K, D> NoGapsRefGapQueryIntervalTree<I, K, D>
where
    I: PointType,
    K: IntervalType<I>,
{
    pub fn new() -> Self {
        Self::default()
    }
}
