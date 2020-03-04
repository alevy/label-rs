use super::Label;

#[derive(Clone)]
pub struct Labeled<D, L> {
    data: D,
    label: L
}

impl<L: Label, D> Labeled<D, L> {
    pub fn label(&self) -> &L {
        &self.label
    }

    pub unsafe fn labeled(data: D, label: L) -> Labeled<D, L> {
        Labeled { data, label }
    }

    pub unsafe fn unlabel(&self, privilege: Option<usize>) -> (&D, &L) {
        match privilege {
            None => {
                (&self.data, &self.label)
            },
            _ => {
                // TODO(alevy): exercise privilege
                (&self.data, &self.label)
            }
        }
    }

    pub unsafe fn unlabel_mut(&mut self, privilege: Option<usize>) -> (&mut D, &L) {
        match privilege {
            None => {
                (&mut self.data, &self.label)
            },
            _ => {
                // TODO(alevy): exercise privilege
                (&mut self.data, &self.label)
            }
        }
    }
}
