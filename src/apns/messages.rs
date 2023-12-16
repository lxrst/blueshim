#[repr(u8)]
pub enum PayloadId {
    KeepAliveReq = 0x0C,
    KeepAliveRes = 0x0D,
    SetState = 0x14,
    SendNotifReq = 0xA,
    SendNotifRes = 0xB,
}

pub struct Payload {
    pub id: u8,
    pub payload: Vec<u8>,
}
