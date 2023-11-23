pub mod client;


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use client::protos;
    use protos::GetBlockDagInfoRequestMessage;
    use protos::kaspad_request::Payload::GetBlockDagInfoRequest;
    
    use super::*;

    #[tokio::test]
    async fn it_works() {
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
}
