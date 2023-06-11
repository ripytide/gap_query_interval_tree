use discrete_range_map::Interval;

/// A GapQueryIntervalTree as described in my paper
pub trait GapQueryIntervalTree<I, T>
where
	T: Copy + Clone,
{
	//optimisation make this a iterator not a vec to save allocation
	fn gap_query(
		&self,
		with_identifier: Option<I>,
		interval: Interval<T>,
	) -> Vec<Interval<T>>;

	fn insert(&mut self, identifier: I, interval: Interval<T>);
	fn remove(&mut self, identifier: I, interval: Interval<T>);

	/// A default implemented convenience method for getting a gap at
	/// a specific point, this is equivalent to calling
	/// [`gap_query()`] with a point interval.
	///
	/// ```
	/// use discrete_range_map::Interval;
	/// use gap_query_interval_tree::{
	/// 	GapQueryIntervalTree, NoGapsRefGapQueryIntervalTree,
	/// };
	///
	/// let mut tree = NoGapsRefGapQueryIntervalTree::new();
	/// tree.insert(5, Interval { start: 3, end: 6 });
	/// tree.insert(9, Interval { start: 12, end: 28 });
    ///
    /// dbg!(&tree);
	///
	/// assert_eq!(
	/// 	tree.gap_at_point(None, 9),
	/// 	Some(Interval { start: 7, end: 11 })
	/// );
	///
	/// //these are equivalent
	/// /*assert_eq!(
	/// 	tree.gap_at_point(None, 9),
	/// 	tree.gap_query(None, Interval { start: 9, end: 9 }).pop()
	/// );*/
	/// ```
	fn gap_at_point(
		&self,
		with_identifier: Option<I>,
		at_point: T,
	) -> Option<Interval<T>> {
		let mut overlapping = self.gap_query(
			with_identifier,
			Interval {
				start: at_point,
				end: at_point,
			},
		);

		assert!(overlapping.is_empty() || overlapping.len() == 1);

		overlapping.pop()
	}
}
