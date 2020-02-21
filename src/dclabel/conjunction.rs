use std::collections::HashSet;

use super::Disjunction;

/// A disjunctions of [Principals](Principal).
#[derive(PartialEq, Eq, Clone)]
pub struct Conjunction(HashSet<Disjunction>);

impl std::fmt::Debug for Conjunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if self.0.is_empty() {
            write!(f, "True")
        } else {
            use std::iter::FromIterator;
            let v: Vec<String> = Vec::from_iter(self.0.iter().map(|x| format!("{:?}", x)));
            write!(f, "{}", v.join(" /\\ "))
        }
    }
}

impl Conjunction {
    pub fn mk_true() -> Self {
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
        if self.0.is_empty() {
            return true;
        }
        // Each disjunction in rhs must be implied by at least one disjunction in rhs
        rhs.0.iter().all(|r| {
            self.0.iter().any(|s| s.implies(r))
        })
    }

    pub fn to_lnf(&mut self) {
        let mut newset: HashSet<Disjunction> = HashSet::new();

        let mut disjs: Vec<Disjunction> = self.0.drain().collect();
        disjs.sort_unstable();

        for d1 in disjs.drain(0..) {
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

impl std::ops::BitOr for Conjunction {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        let mut newset = HashSet::new();

        if rhs.0.is_empty() {
            return self;
        }

        if self.0.is_empty() {
            return rhs;
        }

        for r in rhs.0.iter() {
            for s in self.0.iter() {
                newset.insert(s.clone() | r.clone());
            }
        }
        self.0 = newset;
        self
    }
}

impl std::ops::BitAnd for Disjunction {
    type Output = Conjunction;

    fn bitand(self, rhs: Self) -> Self::Output {
        Conjunction::mk_true().add(self).add(rhs)
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Conjunction {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        let mut r = Conjunction(quickcheck::Arbitrary::arbitrary(g));
        r.to_lnf();
        r
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let Conjunction(ref x) = self;
        let xs = x.shrink();
        let tagged = xs.map(|x| { let mut r = Conjunction(x); r.to_lnf(); r });
        Box::new(tagged)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn false_is_top() {
        let l1 = Conjunction::mk_true().add(Disjunction::mk_false() | "");
        let l2 = Conjunction::mk_true().add(Disjunction::mk_false());
        assert!(l2.implies(&l1), format!("{:?} ==> {:?}", l2, l1));
    }


    #[test]
    fn true_is_bottom() {
        let l1 = Conjunction::mk_true();
        let l2 = Conjunction::mk_true().add(Disjunction::mk_false() | "");
        assert!(l2.implies(&l1), format!("{:?} ==> {:?}", l2, l1));
    }

    #[test]
    fn two_implies_one() {
        let l1 = Conjunction::mk_true().add(Disjunction::mk_false() | "0");
        let l2 = Conjunction::mk_true().add(Disjunction::mk_false() | "").add(Disjunction::mk_false() | "0");
        assert!(l2.implies(&l1), format!("{:?} ==> {:?}", l2, l1));
    }

    #[test]
    fn false_or_not_false_is_not_false() {
        let l1 = Conjunction::mk_true().add(Disjunction::mk_false() | "");
        let l2 = Conjunction::mk_true().add(Disjunction::mk_false());
        assert_eq!(l1.clone() | l2.clone(), l1);
    }

    #[test]
    fn false_or_multi_is_multi() {
        let l1 = Conjunction::mk_true().add(Disjunction::mk_false() | "") & Conjunction::mk_true().add(Disjunction::mk_false() | "0");
        let l2 = Conjunction::mk_true().add(Disjunction::mk_false());
        //assert_eq!(l1.clone() | l2.clone(), l1);
        assert_eq!(l2 | l1.clone(), l1);
    }

    quickcheck! {
        fn or_is_symmetric(c1: Conjunction, c2: Conjunction) -> bool {
            c1.clone() | c2.clone() == c2 | c1
        }
    }
}
