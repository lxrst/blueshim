//!
//! # Apple Push Notification Service (APNs)
//!

#[derive(Deserialize)]
pub(crate) struct ApnsInitBag {
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
}
