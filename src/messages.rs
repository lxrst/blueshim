use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub(crate) struct MsgBplist {
    #[serde(rename = "t")]
    pub plaintext: String,
    #[serde(rename = "x")]
    pub xml: Option<String>,
    #[serde(rename = "p")]
    pub participants: Vec<String>,
    #[serde(rename = "r")]
    pub sender: Option<String>,
    #[serde(rename = "r")]
    pub id: Option<Uuid>,
    #[serde(rename = "gid")]
    pub group_id: Option<Uuid>,
    //pub body: Option<BalloonBody>,
    #[serde(rename = "")]
    pub effect: Option<String>,

    compressed: bool,
    raw: Option<Vec<u8>>,
}
