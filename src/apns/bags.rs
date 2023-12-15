//!
//! Bags contain configuration information required to interact with APNS.
//!

use base64::prelude::{BASE64_STANDARD, Engine};

#[derive(Deserialize)]
pub struct Bag {
    signature: plist::Data,
    certs: Vec<plist::Data>,
    bag: plist::Data,
}
impl Bag {
    pub fn signature(&self) -> &[u8] {
        self.signature.as_ref()
    }
    pub fn bag(&self) -> InnerBag {
        //let dec = BASE64_STANDARD.decode(self.bag.as_ref()).unwrap();
        plist::from_bytes(self.bag.as_ref()).unwrap()
    }
}

#[derive(Deserialize)]
pub struct InnerBag {
    #[serde(rename = "APNSCourierHostname")]
    pub courier_hostname: String,
    #[serde(rename = "APNSVerifiedCourierHostname")]
    pub verified_courier_hostname: String,
    #[serde(rename = "APNSCourierHostcount")]
    pub courier_host_ct: u16,
    #[serde(rename = "ClientConnectionRetryAttempts")]
    pub client_conn_retries: u16,
    #[serde(rename = "APNSCourierStatus")]
    pub courier_status: bool,
    // XXX: what is AWD?
    #[serde(rename = "APNSAWDSlowReceiveThreshold")]
    pub awd_slow_recv_thresh: f32,
    #[serde(rename = "APNSLowPriorityMessageBatchSize")]
    pub low_priority_msg_batch_size: u16,
    #[serde(rename = "APNSActiveInterval")]
    pub active_interval: u16,
}

fn get_bag(url: impl Into<String>, headers: Option<&[(&str, &str)]>) -> Bag {
    let mut req = ureq::get(&url.into());

    if let Some(headers) = headers {
        for (k, v) in headers {
            req = req.set(k, v);
        }
    }

    let rd = req.call().unwrap().into_reader();
    plist::from_reader_xml(rd).unwrap()
}

pub fn apns_bag() -> Bag {
    get_bag("http://init-p01st.push.apple.com/bag", None)
}

pub fn ids_bag() -> Bag {
    get_bag("https://init.ess.apple.com/WebObjects/VCInit.woa/wa/getBag?ix=3", None)
}

pub fn gsa_bag() -> Bag {
    unimplemented!("Grand Slam not yet implemented");
}
