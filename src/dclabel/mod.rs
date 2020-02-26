mod disjunction;
mod conjunction;

pub use disjunction::Disjunction;
pub use conjunction::Conjunction;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DCLabel {
    secrecy: Conjunction,
    integrity: Conjunction,
}

impl DCLabel {
    pub fn new<C: Into<Conjunction>, D: Into<Conjunction>>(secrecy: C, integrity: D) -> Self {
        DCLabel { secrecy: secrecy.into(), integrity: integrity.into() }
    }

    pub fn top() -> Self {
        DCLabel::new(false, true)
    }

    pub fn public() -> Self {
        DCLabel::new(true, true)
    }

    pub fn bottom() -> Self {
        DCLabel::new(true, false)
    }
}

impl super::Label for DCLabel {

    fn join(&self, rhs: &Self) -> Self {
        let secrecy = {
            let mut conj = self.secrecy.clone() & rhs.secrecy.clone();
            conj.to_lnf();
            conj
        };
        let integrity = {
            let mut conj = self.integrity.clone() | rhs.integrity.clone();
            conj.to_lnf();
            conj
        };
        DCLabel {
            secrecy,
            integrity,
        }
    }

    fn meet(&self, rhs: &Self) -> Self {
        let secrecy = {
            let mut conj = self.secrecy.clone() | rhs.secrecy.clone();
            conj.to_lnf();
            conj
        };
        let integrity = {
            let mut conj = self.integrity.clone() & rhs.integrity.clone();
            conj.to_lnf();
            conj
        };
        DCLabel {
            secrecy,
            integrity,
        }
    }

    fn can_flow_to(&self, rhs: &Self) -> bool {
        rhs.secrecy.implies(&self.secrecy) && self.integrity.implies(&rhs.integrity)
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for DCLabel {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        DCLabel { secrecy: Conjunction::arbitrary(g), integrity: Conjunction::arbitrary(g) }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let tagged = self.secrecy.shrink().zip(self.integrity.shrink()).map(|(x, y)| {
            DCLabel { secrecy: x, integrity: y }
        });
        Box::new(tagged)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Label;

    #[test]
    fn bottom_flows_to_public() {
        assert!(DCLabel::bottom().can_flow_to(&DCLabel::public()));
    }

    #[test]
    fn bottom_flows_to_top() {
        assert!(DCLabel::bottom().can_flow_to(&DCLabel::top()));
    }

    #[test]
    fn public_flows_to_top() {
        assert!(DCLabel::public().can_flow_to(&DCLabel::top()));
    }

    #[test]
    fn top_no_flows_to_public() {
        assert!(!DCLabel::top().can_flow_to(&DCLabel::public()));
    }

    #[test]
    fn top_no_flows_to_bottom() {
        assert!(!DCLabel::top().can_flow_to(&DCLabel::bottom()));
    }

    #[test]
    fn public_no_flows_to_bottom() {
        assert!(!DCLabel::public().can_flow_to(&DCLabel::bottom()));
    }

    #[test]
    fn foobar() {
        {
            let l1 = DCLabel::top();
            let l2 = DCLabel::new("", true);
            let ljoin = l1.join(&l2);
            assert_eq!(ljoin, l1);
            assert_eq!(ljoin, l2.join(&l1));
            assert!(l1.can_flow_to(&ljoin), format!("{:?} <= {:?}", l1, ljoin));
            assert!(l2.can_flow_to(&ljoin), format!("{:?} <= {:?}", l2, ljoin));
        }

        {
            let l1 = DCLabel::bottom();
            let l2 = DCLabel::new("", true);
            let ljoin = l1.join(&l2);
            assert_eq!(ljoin, l2);
            assert_eq!(ljoin, l2.join(&l1));
            assert!(l1.can_flow_to(&ljoin), format!("{:?} <= {:?}", l1, ljoin));
            assert!(l2.can_flow_to(&ljoin), format!("{:?} <= {:?}", l2, ljoin));
        }

        {
            let l1 = DCLabel::new("0", true);
            let l2 = DCLabel::new("", true);
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
            let l2 = DCLabel::new("", true);
            let lmeet = l1.meet(&l2);
            assert_eq!(lmeet, l2);
            assert_eq!(lmeet, l2.meet(&l1));
        }

        {
            let l1 = DCLabel::public();
            let l2 = DCLabel::new((Conjunction::mk_false() | "test1" | "test2") & "test3", (Conjunction::mk_false() | "test4" | "test5") & "test6");
            let lmeet = l1.meet(&l2);
            assert_eq!(lmeet, l2.meet(&l1));
            assert!(lmeet.can_flow_to(&l1));
            assert!(lmeet.can_flow_to(&l2));
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
