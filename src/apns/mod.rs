//!
//! # Apple Push Notification Service (APNs)
//!

pub mod bags;
pub mod messages;

use std::{error::Error, io::Write, net::SocketAddr, str::FromStr, sync::Arc};

use bags::apns_bag;
use rand_core::{OsRng, RngCore};
use rustls::{
    pki_types::{DnsName, ServerName},
    ClientConfig, RootCertStore,
};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufStream},
    net::TcpStream,
};
use tokio_rustls::{client::TlsStream, TlsConnector};

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
        host_num = (num % hosts) + 1,
        host_base = bag.courier_hostname,
    )
}

pub struct Conn {
    conn: BufStream<TlsStream<TcpStream>>,
}
impl Conn {
    /// Initialize connection to APNS
    pub async fn init() -> Self {
        let host = random_courier_host();

        println!("Connecting to {host}...");

        match TcpStream::connect(SocketAddr::from_str(&format!("{host}:{COURIER_PORT}")).unwrap())
            .await
        {
            Ok(strm) => {
                let mut certs = RootCertStore::empty();
                let tls_cfg = ClientConfig::builder()
                    .with_root_certificates(certs)
                    .with_no_client_auth();
                let tlsc = TlsConnector::from(Arc::new(tls_cfg));
                let tls_strm = tlsc
                    .connect(ServerName::DnsName(DnsName::try_from(host).unwrap()), strm)
                    .await
                    .unwrap();

                Self {
                    conn: BufStream::new(tls_strm),
                }
            }
            Err(_) => panic!(),
        }
    }
    /// Read a message from APNS
    ///
    /// **APNS message =/= iMessage message**
    pub async fn read_message(&mut self) -> Result<(u8, Vec<u8>), Box<dyn Error>> {
        let conn = &mut self.conn;

        let id = conn.read_u8().await?;
        let len = conn.read_u32().await?;

        let mut payload = Vec::with_capacity(len as usize);
        conn.read_exact(&mut payload).await?;

        Ok((id, payload))
    }
    /// Write a message to APNS
    ///
    /// **APNS message =/= iMessage message**
    pub async fn write_message(&mut self, id: u8, payload: &[u8]) -> Result<(), Box<dyn Error>> {
        let conn = &mut self.conn;

        conn.write_u8(id).await?;
        conn.write_u32(payload.len() as u32).await?;
        conn.write_all(payload).await?;

        Ok(())
    }
}
