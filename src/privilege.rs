
pub trait SpeaksFor {
    fn speaks_for(&self, rhs: &Self) -> bool;
}

pub trait PrivilegeDescription {
    type Label;

    fn downgrade(&self, rhs: &Self::Label) -> Self::Label;
    fn can_flow_to_p(&self, lhs: &Self::Label, rhs: &Self::Label) -> bool;
}

pub struct Privilege<P> {
    inner_desc: P
}

impl<P: SpeaksFor> Privilege<P> {
    pub fn delegate(&self, target: P) -> Option<Self> {
        if self.inner_desc.speaks_for(&target) {
            Some(Privilege { inner_desc: target })
        } else {
            None
        }
    }
}
