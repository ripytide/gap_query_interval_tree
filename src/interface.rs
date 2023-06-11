use discrete_range_map::DiscreteFiniteBounds;

pub type Interval<T> = DiscreteFiniteBounds<T>;

pub trait GapQueryIntervalTree<I, T>
where
	T: Copy + Clone,
{
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

	//optimisation make this a iterator not a vec to save allocation
	fn gap_query(
		&self,
		with_identifier: Option<I>,
		interval: Interval<T>,
	) -> Vec<Interval<T>>;

	fn insert(&mut self, identifier: I, interval: Interval<T>);
	fn remove(&mut self, identifier: I, interval: Interval<T>);
}
