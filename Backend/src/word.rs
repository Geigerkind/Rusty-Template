use crate::Backend;

pub trait Word {
    fn init(&self);
}

impl Word for Backend {
    fn init(&self)
    {

    }
}