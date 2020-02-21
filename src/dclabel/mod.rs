mod disjunction;
mod conjunction;

pub use disjunction::Disjunction;
pub use conjunction::Conjunction;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Component {
    All,
    Component(Conjunction),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DCLabel {
    pub secrecy: Component,
}

impl super::Label for DCLabel {

    fn join(&self, rhs: &Self) -> Self {
        let secrecy = {
            match (self.secrecy.clone(), rhs.secrecy.clone()) {
                (Component::All, _) => Component::All,
                (_, Component::All) => Component::All,
                (Component::Component(x), Component::Component(y)) => {
                    let mut conj = x & y;
                    conj.to_lnf();
                    Component::Component(conj)
                }
            }
        };
        DCLabel {
            secrecy,
        }
    }

    fn meet(&self, rhs: &Self) -> Self {
        let secrecy = {
            match (self.secrecy.clone(), rhs.secrecy.clone()) {
                (Component::All, a) => a,
                (a, Component::All) => a,
                (Component::Component(x), Component::Component(y)) => {
                    let mut conj = x | y;
                    conj.to_lnf();
                    Component::Component(conj)
                }
            }
        };
        DCLabel {
            secrecy,
        }
    }

    fn can_flow_to(&self, rhs: &Self) -> bool {
        match (self.secrecy.clone(), rhs.secrecy.clone()) {
            (_, Component::All) => true,
            (Component::All, y) => y == Component::All,
            (Component::Component(x), Component::Component(y)) => y.implies(&x),
        }
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Component {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        if bool::arbitrary(g) {
            Component::All
        } else {
            Component::Component(quickcheck::Arbitrary::arbitrary(g))
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match *self {
            Component::All => quickcheck::empty_shrinker(),
            Component::Component(ref x) => {
                let xs = x.shrink();
                let tagged = xs.map(|x| Component::Component(x));
                Box::new(tagged)
            }
        }
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for DCLabel {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        DCLabel { secrecy: Component::arbitrary(g) }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let x = self.secrecy.clone();
        let xs = x.shrink();
        let tagged = xs.map(|x| DCLabel { secrecy: x });
        Box::new(tagged)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Label;

    #[test]
    fn foobar() {
        {
            let l1 = DCLabel { secrecy: Component::Component(Conjunction::mk_true().add(Disjunction::mk_false())) };
            let l2 = DCLabel { secrecy: Component::Component(Conjunction::mk_true().add(Disjunction::mk_false() | "")) };
            let ljoin = l1.join(&l2);
            assert_eq!(ljoin, l1);
            assert_eq!(ljoin, l2.join(&l1));
            assert!(l1.can_flow_to(&ljoin), format!("{:?} <= {:?}", l1, ljoin));
            assert!(l2.can_flow_to(&ljoin), format!("{:?} <= {:?}", l2, ljoin));
        }

        {
            let l1 = DCLabel { secrecy: Component::Component(Conjunction::mk_true()) };
            let l2 = DCLabel { secrecy: Component::Component(Conjunction::mk_true().add(Disjunction::mk_false() | "")) };
            let ljoin = l1.join(&l2);
            assert_eq!(ljoin, l2);
            assert_eq!(ljoin, l2.join(&l1));
            assert!(l1.can_flow_to(&ljoin), format!("{:?} <= {:?}", l1, ljoin));
            assert!(l2.can_flow_to(&ljoin), format!("{:?} <= {:?}", l2, ljoin));
        }

        {
            let l1 = DCLabel { secrecy: Component::Component(Conjunction::mk_true().add(Disjunction::mk_false() | "0")) };
            let l2 = DCLabel { secrecy: Component::Component(Conjunction::mk_true().add(Disjunction::mk_false() | "")) };
            let ljoin = l1.join(&l2);
            assert_eq!(ljoin, l2.join(&l1));
            assert!(l1.can_flow_to(&ljoin), format!("{:?} <= {:?}", l1, ljoin));
            assert!(l2.can_flow_to(&ljoin), format!("{:?} <= {:?}", l2, ljoin));
        }
    }

    #[test]
    fn barbaz() {
        {
            let l1 = DCLabel { secrecy: Component::Component(Conjunction::mk_true().add(Disjunction::mk_false())) };
            let l2 = DCLabel { secrecy: Component::Component(Conjunction::mk_true().add(Disjunction::mk_false() | "")) };
            let lmeet = l1.meet(&l2);
            assert_eq!(lmeet, l2);
            assert_eq!(lmeet, l2.meet(&l1));
            //assert!(l1.can_flow_to(&ljoin), format!("{:?} <= {:?}", l1, ljoin));
            //assert!(l2.can_flow_to(&ljoin), format!("{:?} <= {:?}", l2, ljoin));
        }
    }
}
