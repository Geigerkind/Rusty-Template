use rocket::response::content;
use crate::Backend;
use rocket::State;

pub trait Word {
    fn init(&self);
}

impl Word for Backend {
    fn init(&self)
    {

    }
}