use protos::KaspadRequest;
use protos::GetBlockDagInfoRequestMessage;
use protos::kaspad_request::Payload::GetBlockDagInfoRequest;
use protos::rpc_client::RpcClient;

pub mod protos {
    tonic::include_proto!("protowire");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        
        let mut client = RpcClient::connect("grpc://seeder2.kaspad.net:16110").await.unwrap();
        
        let msg: KaspadRequest = KaspadRequest {
            id: 0,
            payload: Some(GetBlockDagInfoRequest(GetBlockDagInfoRequestMessage {})),
        };

        let (s, r) = async_channel::unbounded();
        let _ = s.send(msg).await;

        let stream_request = async_stream::stream! {
            while let Ok(msg) = r.recv().await {
                yield msg;
            }
        };

        let request = tonic::Request::new(stream_request);
        let respone_stream = client.message_stream(request).await.unwrap();
        
        let _ = tokio::spawn(async move {
            let mut respone_stream = respone_stream.into_inner();
            while let Some(msg) = respone_stream.message().await.unwrap() {
                if let Some(payload) = msg.payload {
                    match payload {
                        protos::kaspad_response::Payload::GetBlockDagInfoResponse(resp_body) => {
                            println!("GetBlockDagInfoResponse: {:?}", resp_body);
                        },
                        _ => {
                            println!("Other");
                        }
                        
                    }
                }
            }
        });
        
        tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
        
    }
}
