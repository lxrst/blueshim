#![doc = include_str!("../README.md")]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;
extern crate base64;
extern crate plist;
extern crate rsa;
extern crate rustls;
extern crate rustls_pemfile;
extern crate tokio;
extern crate x509_cert;

pub mod albert;
pub mod apns;
pub mod gsa;
pub mod ids;
pub mod messages;
//mod emulation;

pub struct Blueshim {}
impl Blueshim {
    pub fn init() -> Self {
        Self {}
    }
}
