use std::fmt::Display;

use ureq::Request;
use uuid::Uuid;

pub const USER_AGENT: &'static str = "akd/1.0 CFNetwork/978.0.7 Darwin/18.7.0";

/// A set of Grandslam client information
///
/// Apple uses this for DRM.
pub struct GsaClient<'a> {
    pub model: &'a str,
    pub os: &'a str,
    pub os_version: &'a str,
    pub authkit_bundle: &'a str,
    pub authkit_version: &'a str,
    pub app_bundle: &'a str,
    pub app_version: &'a str,
}
impl<'a> Default for GsaClient<'a> {
    fn default() -> Self {
        Self {
            model: "MacBookPro18,3",
            os: "Mac OS X",
            os_version: "13.4.1;22F8",
            app_bundle: "com.apple.accountsd",
            app_version: "113",
            authkit_bundle: "com.apple.AOSKit",
            authkit_version: "282",
        }
    }
}
impl<'a> Display for GsaClient<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "<{}> <{};{}> <{}/{} ({}/{})>",
            self.model,
            self.os,
            self.os_version,
            self.authkit_bundle,
            self.authkit_version,
            self.app_bundle,
            self.app_version,
        )
    }
}

pub fn gen_meta_headers(req: Request, serial: &str, user_id: Uuid, device_id: Uuid) -> Request {
    req.set("X-Apple-I-Client-Time", "")
        .set("X-Apple-I-TimeZone", "")
        .set("loc", "en_US")
        .set("X-Apple-Locale", "en_US")
        .set("X-Apple-I-MD-RINFO", "17106176")
        .set("X-Apple-I-MD-LU", "")
        .set("X-Mme-Device-Id", &device_id.to_string().to_uppercase())
        .set("X-Apple-I-SRL-NO", serial)
}
