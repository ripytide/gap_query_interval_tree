A crate that provides a gap-query optimized interval-tree
data-structure.

There are three main operations available on this data-structure:
insertion, removal and gap-queries. Each of which are `O(log(N) + K)`
where `N` is the total number of intervals in the tree and `K` is the
number of intervals required to be processed.

Here are visualizations of the three operations:

# Insertion

![insertion](images/insertion.svg)

# Removal

![removal](images/removal.svg)

# Gap-Query

![gap-query](images/gap-query.svg)
