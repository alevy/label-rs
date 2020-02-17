use std::collections::BTreeSet;
use std::fmt;

pub type Principal = String;

/// A disjunctions of [Principals](Principal).
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Disjunction(BTreeSet<Principal>);

impl std::hash::Hash for Disjunction {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        for p in self.0.iter() {
            p.hash(hasher);
        }
    }
}

impl Disjunction {
    pub fn empty() -> Self {
        Disjunction(BTreeSet::new())
    }

    /// Add a principal to the disjunction
    pub fn add<P: Into<Principal>>(mut self, principal: P) -> Self {
        self.0.insert(principal.into());
        self
    }

    /// The disjunction implies another disjunction
    ///
    /// Returns true if the disjunction contains a subset of the principals present in rhs, or if
    /// rhs is false (contains no principals).
    pub fn implies(&self, rhs: &Self) -> bool {
        if rhs.0.is_empty() {
            true // everything implies false
        } else if self.0.is_empty() {
            false // false can't imply anything other than false
        } else {
            // rhs must be a superset of self
            self.0.iter().all(|i| {
                rhs.0.contains(i)
            })
        }
    }
}

impl<I: Into<String>> From<I> for Disjunction {
    fn from(s: I) -> Self {
        let mut r = BTreeSet::new();
        r.insert(s.into());
        Disjunction(r)
    }
}

impl std::ops::BitOr for Disjunction {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        self.0.union(&rhs.0);
        self
    }
}

impl<P: Into<Principal>> std::ops::BitOr<P> for Disjunction {
    type Output = Self;

    fn bitor(self, rhs: P) -> Self {
        self.add(rhs)
    }
}

impl fmt::Display for Disjunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        let mut iter = self.0.iter();
        if let Some(head) = iter.next() {
            head.fmt(f)?;
            for p in iter {
                write!(f, " \\/ {}", p)?;
            }
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_false() {
        let d = Disjunction::empty();

        assert_eq!(format!("{}", d), "()");
    }

    #[test]
    fn display_one_principal() {
        let d = Disjunction::empty() | "foo";

        assert_eq!(format!("{}", d), "(foo)");
    }

    #[test]
    fn display_two_principals() {
        let d = Disjunction::empty() | "foo" | "bar";

        assert_eq!(format!("{}", d), "(bar \\/ foo)");
    }

    #[test]
    fn display_three_principals() {
        let d = Disjunction::empty() | "foo" | "bar" | "baz";

        assert_eq!(format!("{}", d), "(bar \\/ baz \\/ foo)");
    }

    #[test]
    fn false_implies_false() {
        let d0 = Disjunction::empty();
        let d1 = Disjunction::empty();

        assert!(d1.implies(&d0));
    }

    #[test]
    fn non_false_implies_false() {
        let d0 = Disjunction::empty();
        let d1 = Disjunction::empty() | "foo";

        assert!(d1.implies(&d0));
    }

    #[test]
    fn false_doesnt_imply_non_false() {
        let d0 = Disjunction::empty() | "foo";
        let d1 = Disjunction::empty();

        assert!(!d1.implies(&d0), "false should not imply non-false values");
    }

    quickcheck! {
        fn same_implies_same(principals: Vec<Principal>) -> bool {
            let mut d0 = Disjunction::empty();
            for p in principals.iter() {
                d0 = d0 | p;
            }

            let mut d1 = Disjunction::empty();
            for p in principals.iter() {
                d1 = d1 | p;
            }

            d1.implies(&d0)
        }

        fn subset_implies_superset(principals: Vec<Principal>, shared: usize) -> bool {
            let mut d0 = Disjunction::empty();
            for p in principals.iter() {
                d0 = d0 | p;
            }

            let mut d1 = Disjunction::empty();
            for p in principals.iter().take(shared + 1) {
                d1 = d1 | p;
            }

            d1.implies(&d0)
        }

        fn subset_not_implied_by_superset(principals: Vec<Principal>, shared: usize) -> bool {
            let mut d0 = Disjunction::empty() | "foobar";
            for p in principals.iter() {
                d0 = d0 | p;
            }

            let mut d1 = Disjunction::empty() | "barbaz";
            for p in principals.iter().take(shared) {
                d1 = d1 | p;
            }

            !d0.implies(&d1)
        }
    }
}
