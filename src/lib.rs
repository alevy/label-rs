#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod disjunction;
mod conjunction;

pub use disjunction::Disjunction;
pub use conjunction::Conjunction;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Component {
    All,
    Component(Conjunction),
}

pub struct DCLabel {
    secrecy: Component,
}

impl DCLabel {

}

#[cfg(test)]
mod tests {
}
