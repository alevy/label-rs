use std::collections::BTreeSet;
use std::fmt;

pub type Principal = String;

/// A disjunctions of [Principals](Principal).
#[derive(PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Disjunction(BTreeSet<Principal>);

impl std::fmt::Debug for Disjunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if self.0.is_empty() {
            write!(f, "False")
        } else {
            use std::iter::FromIterator;
            let v: Vec<String> = Vec::from_iter(self.0.iter().map(|x| format!("{:?}", x)));
            write!(f, "{}", v.join(" \\/ "))
        }
    }
}

impl std::hash::Hash for Disjunction {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        for p in self.0.iter() {
            p.hash(hasher);
        }
    }
}

impl Disjunction {
    pub fn mk_false() -> Self {
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
        if self.0.is_empty() {
            true // if false is true then anything is true
        } else if rhs.0.is_empty() {
            false // only false implies false
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

impl std::ops::BitOr for &Disjunction {
    type Output = Disjunction;

    fn bitor(self, rhs: Self) -> Disjunction {
        Disjunction(&self.0 | &rhs.0)
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
impl quickcheck::Arbitrary for Disjunction {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        Disjunction(quickcheck::Arbitrary::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let Disjunction(ref x) = self;
        let xs = x.shrink();
        let tagged = xs.map(|x| Disjunction(x));
        Box::new(tagged)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_false() {
        let d = Disjunction::mk_false();

        assert_eq!(format!("{}", d), "()");
    }

    #[test]
    fn display_one_principal() {
        let d = Disjunction::mk_false() | "foo";

        assert_eq!(format!("{}", d), "(foo)");
    }

    #[test]
    fn display_two_principals() {
        let d = Disjunction::mk_false() | "foo" | "bar";

        assert_eq!(format!("{}", d), "(bar \\/ foo)");
    }

    #[test]
    fn display_three_principals() {
        let d = Disjunction::mk_false() | "foo" | "bar" | "baz";

        assert_eq!(format!("{}", d), "(bar \\/ baz \\/ foo)");
    }

    #[test]
    fn false_implies_false() {
        let d0 = Disjunction::mk_false();
        let d1 = Disjunction::mk_false();

        assert!(d1.implies(&d0));
    }

    #[test]
    fn non_false_doesnt_imply_false() {
        let d0 = Disjunction::mk_false();
        let d1 = Disjunction::mk_false() | "foo";

        assert!(!d1.implies(&d0));
    }

    #[test]
    fn false_implies_non_false() {
        let d0 = Disjunction::mk_false() | "foo";
        let d1 = Disjunction::mk_false();

        assert!(d1.implies(&d0), "false should imply non-false values");
    }

    #[test]
    fn false_or_not_false_is_not_false() {
        let d0 = Disjunction::mk_false() | "0";
        let d1 = Disjunction::mk_false();

        assert_eq!(&d1 | &d0, d0);
    }

    quickcheck! {
        fn same_implies_same(principals: Vec<Principal>) -> bool {
            let mut d0 = Disjunction::mk_false();
            for p in principals.iter() {
                d0 = d0 | p;
            }

            let mut d1 = Disjunction::mk_false();
            for p in principals.iter() {
                d1 = d1 | p;
            }

            d1.implies(&d0)
        }

        fn subset_implies_superset(principals: Vec<Principal>, shared: usize) -> bool {
            let mut d0 = Disjunction::mk_false();
            for p in principals.iter() {
                d0 = d0 | p;
            }

            let mut d1 = Disjunction::mk_false();
            for p in principals.iter().take(shared + 1) {
                d1 = d1 | p;
            }

            d1.implies(&d0)
        }

        fn subset_not_implied_by_superset(principals: Vec<Principal>, shared: usize) -> bool {
            let mut d0 = Disjunction::mk_false() | "foobar";
            for p in principals.iter() {
                d0 = d0 | p;
            }

            let mut d1 = Disjunction::mk_false() | "barbaz";
            for p in principals.iter().take(shared) {
                d1 = d1 | p;
            }

            !d0.implies(&d1)
        }

        fn or_is_symmetric(c1: Disjunction, c2: Disjunction) -> bool {
            let c1 = &c1;
            let c2 = &c2;
            c1 | c2 == c2 | c1
        }
    }
}
