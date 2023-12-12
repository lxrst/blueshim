#[macro_use]
extern crate serde;

mod albert;
mod apns;
mod messages;
mod ids;
//mod emulation;

pub struct Blueshim {
}
impl Blueshim {
    pub fn init() -> Self {
        Self {}
    }
}
