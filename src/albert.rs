//!
//! Albert (`albert.apple.com`) seems to manage device registration for a variety of Apple things.
//!
//! It makes use of Fairplay, Apple's DRM system, to lock things down.
//!
//! ## Notes
//!
//! `pypush`, as of the time of writing, leverages what appears to be iTunes for Windows'
//! activation info (and probably keys?).
//! This could be a fairly easy patch on Apple's side, as I don't believe iTunes for Windows needs to interact with APNs, much less iMessage.
//!
//! The exponent `65537` is used a lot for private key generation.
//!

use uuid::Uuid;
use rsa::{RsaPrivateKey, rand_core::OsRng, BigUint};

fn gen_csr(priv_key: RsaPrivateKey) {
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct ActivationInfo {
    pub activation_randomness: Uuid,
    pub activation_state: &'static str,
    pub build_version: &'static str,
    pub device_cert_request: String,
    pub device_class: &'static str,
    pub product_type: &'static str,
    pub product_version: &'static str,
    pub serial_number: &'static str,
    #[serde(rename = "UniqueDeviceID")]
    pub unique_device_id: Uuid,
}

fn gen_push_cert() {
    match RsaPrivateKey::new_with_exp(&mut OsRng, 2048, &BigUint::from(65537u64)) {
        Ok(priv_key) => {
            // gen csr w/ priv_key

            let info = &ActivationInfo {
                activation_randomness: Uuid::new_v4(),
                activation_state: "Unactivated",
                build_version: "10.6.4",
                device_cert_request: String::default(),
                device_class: "Windows",
                product_type: "windows1,1",
                product_version: "10.6.4",
                serial_number: "WindowsSerial",
                unique_device_id: Uuid::new_v4(),
            };

            let info_plist = &mut Vec::new();
            plist::to_writer_xml(info_plist, info);

            // more junk
        }
        Err(err) => {}
    }
}
