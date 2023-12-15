#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;
extern crate tokio;
extern crate http;
extern crate rsa;
extern crate x509_cert;
extern crate plist;
extern crate rustls;
extern crate rustls_pemfile;
extern crate base64;

pub mod albert;
pub mod apns;
pub mod messages;
pub mod ids;
//mod emulation;

pub struct Blueshim {
}
impl Blueshim {
    pub fn init() -> Self {
        Self {}
    }
}
