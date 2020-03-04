use std::cell::RefCell;

use super::Label;
use super::dclabel::DCLabel;

thread_local! {
    static CURRENT_LABEL: RefCell<DCLabel> = RefCell::new(DCLabel::public());
}

pub fn current_label() -> DCLabel {
    CURRENT_LABEL.with(|cl| {
        cl.borrow().clone()
    })
}

pub fn taint(label: &DCLabel) {
    CURRENT_LABEL.with(|cl| {
        let mut my_cl = cl.borrow_mut();
        *my_cl = my_cl.join(label);
    });
}

pub fn guard_alloc(label: &DCLabel) {
    CURRENT_LABEL.with(|cl| {
        let my_cl = cl.borrow();
        if !my_cl.can_flow_to(label) {
            panic!("Guard alloc failed");
        }
    })
}

pub fn guard_write(label: &DCLabel) {
    guard_alloc(label);
    taint(label);
}

pub trait LabeledRead<D> {
    fn unlabel_read(&self) -> &D;
}

impl<D> LabeledRead<D> for super::labeled::Labeled<D, DCLabel> {
    fn unlabel_read(&self) -> &D {
        let (d, l) = unsafe { self.unlabel(None) };
        taint(l);
        d
    }
}

pub trait LabeledWrite<D> {
    fn unlabel_write(&mut self) -> &mut D;
}

impl<D> LabeledWrite<D> for super::labeled::Labeled<D, DCLabel> {
    fn unlabel_write(&mut self) -> &mut D {
        let (d, l) = unsafe { self.unlabel_mut(None) };
        guard_write(l);
        d
    }
}

