//!
//! iMessage Identity Server
//!

use std::io::{Write, Read};

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Registration {
    pub hardware_version: &'static str,
    pub language: &'static str,
    pub os_version: &'static str,
    pub software_version: &'static str,
    pub services: RegistrationServices,
    pub validation_data: String,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RegistrationServices {
    pub capabilities: RegistrationCapabilities,
    pub service: &'static str,
    pub users: Vec<RegistrationUser>,
}

#[derive(Serialize)]
pub struct RegistrationCapabilities {
    pub flags: u8,
    pub name: &'static str,
    pub version: u8,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RegistrationUser {
    pub client_data: RegistrationClientData,
    pub uris: String,
    pub user_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RegistrationClientData {
    pub is_c2k_equipment: bool,
    pub optionally_receive_typing_indicators: bool,
    pub public_message_identity_key: String, // identity.encode()
    pub public_message_identity_version: u8,
    pub show_peer_error: bool,
    pub supports_ack_v1: bool,
    pub supports_activity_sharing_v1: bool,
    pub supports_audio_messaging_v2: bool,
    pub supports_autoloopvideo_v1: bool,
    pub supports_be_v1: bool,
    pub supports_ca_v1: bool,
    pub supports_fsm_v1: bool,
    pub supports_fsm_v2: bool,
    pub supports_fsm_v3: bool,
    pub supports_ii_v1: bool,
    pub supports_impact_v1: bool,
    pub supports_inline_attachments: bool,
    pub supports_keep_receipts: bool,
    pub supports_location_sharing: bool,
    pub supports_media_v2: bool,
    pub supports_photos_extension_v1: bool,
    pub supports_st_v1: bool,
    pub supports_update_attachments_v1: bool,
}
 
/*
pub fn register() {
    let r = Registration {
        hardware_version: "MacBookPro18,3",
        language: "en-US",
        os_version: "macOS,13.2.1,22D68",
        software_version: "22D68",
        validation_data: String::default(), // b64 dec val data
    };
}
*/

pub fn decode<R: Read>(input: &mut R) {
    {
        let mut buf = [0u8; 5];
        input.read_exact(&mut buf).unwrap();
        assert_eq!(
            buf,
            [0x30, 0x81, 0xF6, 0x81, 0x43],
            "invalid DER header",
        );
    }

    let mut raw_ecdsa = [0u8; 67];
    input.read_exact(&mut raw_ecdsa).unwrap();

    {
        let mut buf = [0u8; 3];
        input.read_exact(&mut buf).unwrap();
        assert_eq!(
            buf,
            [0x82, 0x81, 0xAE],
            "invalid DER header",
        );
    }

}
