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
    secrecy: Component,
}

impl super::Label for DCLabel {

    fn join(&self, rhs: &Self) -> Self {
        self.clone()
    }

    fn meet(&self, rhs: &Self) -> Self {
        self.clone()
    }

    fn can_flow_to(&self, rhs: &Self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
}
