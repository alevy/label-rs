use crate::Label;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TwoLevel {
    Low,
    High
}

impl Label for TwoLevel {
    fn join(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (TwoLevel::High, _) => TwoLevel::High,
            (_, TwoLevel::High) => TwoLevel::High,
            _ => TwoLevel::Low,
        }
    }

    fn meet(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (TwoLevel::Low, _) => TwoLevel::Low,
            (_, TwoLevel::Low) => TwoLevel::Low,
            _ => TwoLevel::High,
        }
    }

    fn can_flow_to(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (TwoLevel::High, TwoLevel::High) => true,
            (TwoLevel::Low, _) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for TwoLevel {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        if bool::arbitrary(g) {
            TwoLevel::Low
        } else {
            TwoLevel::High
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn join_low() {
        assert_eq!(TwoLevel::Low.join(&TwoLevel::Low), TwoLevel::Low);
        assert_eq!(TwoLevel::Low.join(&TwoLevel::High), TwoLevel::High);
    }

    #[test]
    fn join_high() {
        assert_eq!(TwoLevel::High.join(&TwoLevel::Low), TwoLevel::High);
        assert_eq!(TwoLevel::High.join(&TwoLevel::High), TwoLevel::High);
    }

    #[test]
    fn meet_low() {
        assert_eq!(TwoLevel::Low.meet(&TwoLevel::Low), TwoLevel::Low);
        assert_eq!(TwoLevel::Low.meet(&TwoLevel::High), TwoLevel::Low);
    }

    #[test]
    fn meet_high() {
        assert_eq!(TwoLevel::High.meet(&TwoLevel::Low), TwoLevel::Low);
        assert_eq!(TwoLevel::High.meet(&TwoLevel::High), TwoLevel::High);
    }

    #[test]
    fn low_can_flow_to() {
        assert!(TwoLevel::Low.can_flow_to(&TwoLevel::Low));
        assert!(TwoLevel::Low.can_flow_to(&TwoLevel::High));
    }

    #[test]
    fn high_can_flow_to_high() {
        assert!(TwoLevel::High.can_flow_to(&TwoLevel::High));
    }

    #[test]
    fn high_cannot_flow_to_low() {
        assert!(!TwoLevel::High.can_flow_to(&TwoLevel::Low));
    }
}
