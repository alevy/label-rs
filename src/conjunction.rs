use std::collections::HashSet;

use crate::Disjunction;

/// A disjunctions of [Principals](Principal).
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Conjunction(HashSet<Disjunction>);

impl Conjunction {
    pub fn empty() -> Self {
        Conjunction(HashSet::new())
    }

    /// Add a disjunction clause
    ///
    /// Only adds the disjunction if no other disjunction implies it.  Simplifies the conjunction
    /// be removing any existing disjunctions implied by the new disjunction.
    pub fn add(mut self, disj: Disjunction) -> Self {
        if self.0.iter().any(|d| d.implies(&disj)) {
            return self
        }

        self.0.retain(|d| !disj.implies(d));
        self.0.insert(disj);
        self
    }

    /// The disjunction implies another disjunction
    ///
    /// Returns true if the conjunctions contains a superset of the principals present in rhs.
    pub fn implies(&self, rhs: &Self) -> bool {
        // Each disjunction in rhs must be implied by at least one disjunction in self
        rhs.0.iter().all(|i| {
            self.0.iter().any(|j| i.implies(j))
        })
    }

    pub fn to_lnf(&mut self) {
        let mut newset: HashSet<Disjunction> = HashSet::new();
        for d1 in self.0.drain() {
            if !newset.iter().any(|d0| d0.implies(&d1)) {
                newset.insert(d1);
            }
        }
        self.0 = newset;
    }
}

impl std::ops::BitAnd for Conjunction {
    type Output = Self;

    fn bitand(mut self, mut rhs: Self) -> Self::Output {
        for d in rhs.0.drain() {
            self = self.add(d);
        }
        self
    }
}

impl std::ops::BitAnd for Disjunction {
    type Output = Conjunction;

    fn bitand(self, rhs: Self) -> Self::Output {
        Conjunction::empty().add(self).add(rhs)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn fail() {
        assert!(true);
    }
}
