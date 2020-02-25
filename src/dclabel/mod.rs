mod disjunction;
mod conjunction;

pub use disjunction::Disjunction;
pub use conjunction::Conjunction;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DCLabel {
    secrecy: Conjunction,
}

impl DCLabel {
    pub fn new<C: Into<Conjunction>>(secrecy: C) -> Self {
        DCLabel { secrecy: secrecy.into() }
    }

    pub fn top() -> Self {
        DCLabel::new(false)
    }

    pub fn bottom() -> Self {
        DCLabel::new(true)
    }
}

impl super::Label for DCLabel {

    fn join(&self, rhs: &Self) -> Self {
        let secrecy = {
            let mut conj = self.secrecy.clone() & rhs.secrecy.clone();
            conj.to_lnf();
            conj
        };
        DCLabel {
            secrecy,
        }
    }

    fn meet(&self, rhs: &Self) -> Self {
        let secrecy = {
            let mut conj = self.secrecy.clone() | rhs.secrecy.clone();
            conj.to_lnf();
            conj
        };
        DCLabel {
            secrecy,
        }
    }

    fn can_flow_to(&self, rhs: &Self) -> bool {
        rhs.secrecy.implies(&self.secrecy)
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for DCLabel {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        DCLabel { secrecy: Conjunction::arbitrary(g) }
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
            let l1 = DCLabel::top();
            let l2 = DCLabel::new("");
            let ljoin = l1.join(&l2);
            assert_eq!(ljoin, l1);
            assert_eq!(ljoin, l2.join(&l1));
            assert!(l1.can_flow_to(&ljoin), format!("{:?} <= {:?}", l1, ljoin));
            assert!(l2.can_flow_to(&ljoin), format!("{:?} <= {:?}", l2, ljoin));
        }

        {
            let l1 = DCLabel::bottom();
            let l2 = DCLabel::new("");
            let ljoin = l1.join(&l2);
            assert_eq!(ljoin, l2);
            assert_eq!(ljoin, l2.join(&l1));
            assert!(l1.can_flow_to(&ljoin), format!("{:?} <= {:?}", l1, ljoin));
            assert!(l2.can_flow_to(&ljoin), format!("{:?} <= {:?}", l2, ljoin));
        }

        {
            let l1 = DCLabel::new("0");
            let l2 = DCLabel::new("");
            let ljoin = l1.join(&l2);
            assert_eq!(ljoin, l2.join(&l1));
            assert!(l1.can_flow_to(&ljoin), format!("{:?} <= {:?}", l1, ljoin));
            assert!(l2.can_flow_to(&ljoin), format!("{:?} <= {:?}", l2, ljoin));
        }
    }

    #[test]
    fn barbaz() {
        {
            let l1 = DCLabel::top();
            let l2 = DCLabel::new("");
            let lmeet = l1.meet(&l2);
            assert_eq!(lmeet, l2);
            assert_eq!(lmeet, l2.meet(&l1));
        }

        {
            let l1 = DCLabel::top();
            let l2 = DCLabel::bottom();
            let lmeet = l1.meet(&l2);
            assert_eq!(lmeet, l2);
            assert_eq!(lmeet, l2.meet(&l1));
        }
    }
}
