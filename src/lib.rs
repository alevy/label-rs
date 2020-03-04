#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub mod labeled;
pub mod privilege;
pub mod runtime;

pub mod dclabel;
pub mod twolevel;

#[cfg(test)]
mod qc_tests;

/// A `Label` behaves as a [lattice](https://en.wikipedia.org/wiki/Lattice_(order)).
///
/// It defines a partially ordered set where each pair of values has a `join` (or least upper
/// bound), `meet` (or greatest lower bound), and a partial order (`can_flow_to`). These operations
/// can express information flow relationships between labels.
pub trait Label {

    /// Compute the least upper bound of two labels.
    ///
    /// When combining data with two different labels, the _join_ is the lowest safe label for the
    /// result.
    ///
    /// For two labels `l1` and `l2`, if `l_join = l1.join(l2)`:
    ///
    ///   * `l_join == l2.join(l1)`
    ///   * `l1.can_flow_to(l_join) == true`
    ///   * `l2.can_flow_to(l_join) == true`, and
    ///   * no label `l != l_join` exists s.t. `l1.can_flow_to(l) && l2.can_flow_to(l) &&
    ///   l_join.can_flow_to(l)`.
    fn join(&self, rhs: &Self) -> Self;

    /// Compute the greatest lower bound of two labels.
    ///
    /// The `meet` of two labels is the greatest element that can flow to both labels.
    ///
    /// For two labels `l1` and `l2`, if `l_meet = l1.meet(l2)`:
    ///
    ///   * `l_meet == l2.meet(l1)`
    ///   * `l_meet.can_flow_to(l1) == true`
    ///   * `l_meet.can_flow_to(l2) == true`, and
    ///   * no label `l != l_meet` exists s.t. `l.can_flow_to(l1) && l.can_flow_to(l2) &&
    ///   l_meet.can_flow_to(l)`.
    fn meet(&self, rhs: &Self) -> Self;

    /// Can-flow-to relation (⊑).
    ///
    /// An entity labeled `l1` should be allowed to affect an entity `l2` only if
    /// `l1.can_flow_to(l2)`. This relation on labels is at least a [partial
    /// order](https://en.wikipedia.org/wiki/Partially_ordered_set), and must satisfy the following
    /// laws:
    ///
    ///   * Reflexivity: `l1.can_flow_to(l1)` for any l1.
    ///   * Antisymmetry: If `l1.can_flow_to(l2) && l2.can_flow_to(l1)` then `l1 == l2`.
    ///   * Transitivity: If `l1.can_flow_to(l2) && l2.can_flow_to(l3)` then `l1.can_flow_to(l3)`.
    fn can_flow_to(&self, rhs: &Self) -> bool;
}

#[cfg(test)]
mod tests {
}
