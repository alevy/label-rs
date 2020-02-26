// DCLabel
mod dcl {
    use crate::Label;
    use crate::dclabel::*;
    quickcheck! {
        fn join(l1: DCLabel, l2: DCLabel) -> bool {
            let ljoin = l1.join(&l2);

            ljoin == l2.join(&l1) &&
                l1.can_flow_to(&ljoin) &&
                l2.can_flow_to(&ljoin)
        }

        fn meet(l1: DCLabel, l2: DCLabel) -> bool {
            let lmeet = l1.meet(&l2);

            lmeet == l2.meet(&l1) &&
                lmeet.can_flow_to(&l1) &&
                lmeet.can_flow_to(&l2)
        }
    }
}

// Twolevel
mod tl {
    use crate::Label;
    use crate::twolevel::*;

    quickcheck! {
        fn join(l1: TwoLevel, l2: TwoLevel) -> bool {
            let ljoin = l1.join(&l2);

            ljoin == l2.join(&l1) &&
                l1.can_flow_to(&ljoin) &&
                l2.can_flow_to(&ljoin) &&
                (ljoin == TwoLevel::High || !TwoLevel::High.can_flow_to(&ljoin))
        }

        fn meet(l1: TwoLevel, l2: TwoLevel) -> bool {
            let lmeet = l1.meet(&l2);

            lmeet == l2.meet(&l1) &&
                lmeet.can_flow_to(&l1) &&
                lmeet.can_flow_to(&lmeet) &&
                (lmeet == TwoLevel::Low || !lmeet.can_flow_to(&TwoLevel::Low))
        }
    }
}
