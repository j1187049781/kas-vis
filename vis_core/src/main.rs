use std::sync::Arc;
use kas_grpc::client;
use kas_grpc::client::protos;
use protos::GetBlockDagInfoRequestMessage;
use protos::kaspad_request::Payload::GetBlockDagInfoRequest;
#[tokio::main]
async fn main() {
    let c = client::KaspadClient::new(String::from("grpc://seeder2.kaspad.net:16110"));
    let ac = Arc::new(c);
    ac.clone().connect().await.unwrap();
    let payload = GetBlockDagInfoRequest(GetBlockDagInfoRequestMessage {});
    let resp = ac.clone().get(payload).await;
    if let Ok(msg) = resp {
        if let Some(payload) = msg.payload {
            print!("payload: {payload:?}")
        }
    }
}