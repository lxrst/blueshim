#[tokio::main]
async fn main() {
    blueshim::albert::gen_push_cert();
    blueshim::apns::Conn::init().await;
}
