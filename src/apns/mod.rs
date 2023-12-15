//!
//! # Apple Push Notification Service (APNs)
//!

pub mod bags;
use std::{net::SocketAddr, str::FromStr, sync::Arc, io::Write};

use bags::apns_bag;
use rand_core::{OsRng, RngCore};
use rustls::{ClientConfig, RootCertStore, pki_types::{ServerName, DnsName}};
use tokio::{net::TcpStream, io::AsyncWrite};
use tokio_rustls::{TlsConnector, client::TlsStream, TlsStream};

/// The port Apple's courier server listens on
///
/// This *shouldn't* change, but I'll make this a constant in case it does.
const COURIER_PORT: u16 = 5223;

const ALPN: &'static [&'static str] = &["apns-security-v3"];

fn random_courier_host() -> String {
    // Fetch the APNS bag from Apple
    let bag = apns_bag().bag();
    // Generate a random number we'll use to pick a courier host
    let num = OsRng::default().next_u32();
    // Get courier host count
    let hosts = bag.courier_host_ct as u32;

    format!(
        "{host_num}-{host_base}",
        host_num = (num % hosts)+1,
        host_base = bag.courier_hostname,
    )
}

pub struct Conn {
    conn: TlsStream<TcpStream>,
}
impl Conn {
    pub async fn init() {
        let host = random_courier_host();

        println!("Connecting to {host}...");

        let mut strm = TcpStream::connect(SocketAddr::from_str(&format!("{host}:{COURIER_PORT}")).unwrap()).await.unwrap();

        let mut certs = RootCertStore::empty();
        let tls_cfg = ClientConfig::builder().with_root_certificates(certs).with_no_client_auth();
        let tlsc = TlsConnector::from(Arc::new(tls_cfg));
        let mut tls_strm = tlsc.connect(ServerName::DnsName(DnsName::try_from(host).unwrap()), strm).await.unwrap();
        let (io, conn) = tls_strm.get_mut();
    }
    pub async fn write_payload(&mut self, id: u8, payload: &[u8]) {
        let mut buf = Vec::new();

        buf.write_all(&id.to_be_bytes()).unwrap();
        buf.write_all(&(payload.len() as u32).to_be_bytes()).unwrap();
        buf.write_all(payload).unwrap();

        self.conn.get_mut().1.writer().write_all(&buf).unwrap();
    }
}
