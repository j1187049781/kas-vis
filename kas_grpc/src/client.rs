use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use async_channel::Receiver;
use async_channel::Sender;
use protos::KaspadRequest;
use protos::GetBlockDagInfoRequestMessage;
use protos::kaspad_request::Payload::GetBlockDagInfoRequest;
use protos::rpc_client::RpcClient;
use protos::KaspadResponse;
use tonic::Streaming;
use anyhow;

use self::protos::kaspad_request::Payload;



pub mod protos {
    tonic::include_proto!("protowire");
}

pub struct KaspadClient {
    sender: Sender<KaspadRequest>,
    receiver: Receiver<KaspadRequest>,
    stream: Option<Streaming<KaspadResponse>>,
    req_id: AtomicU64,
    host: String,
}

impl KaspadClient {
    pub fn new(host: String) -> Self {
        let (s, r) = async_channel::unbounded();
        return KaspadClient{
            sender: s,
            receiver: r,
            stream: None,
            req_id: AtomicU64::new(1),
            host
        };
    }

    pub async fn get<T>(self: &mut Self, payload: Payload) -> Result<(),anyhow::Error>
    {   let req_id = self.req_id.fetch_add(1, Relaxed);
        let msg: KaspadRequest = KaspadRequest {
            id: req_id ,
            payload: Some(payload),
        };
        self.sender.send(msg).await?;

        Ok(())
    }

    pub async fn connect(self: &mut Self ) -> Result<(), anyhow::Error> {
        let mut client = RpcClient::connect(self.host.clone()).await.unwrap();
        
        let msg: KaspadRequest = KaspadRequest {
            id: 0,
            payload: Some(GetBlockDagInfoRequest(GetBlockDagInfoRequestMessage {})),
        };

        self.sender.send(msg).await?;

        let r = self.receiver.clone();
        let stream_request = async_stream::stream! {
            while let Ok(msg) = r.recv().await {
                yield msg;
            }
        };

        let request = tonic::Request::new(stream_request);
        let respone_stream = client.message_stream(request).await.unwrap();
        
        let mut respone_stream = respone_stream.into_inner();
        while let Some(msg) = respone_stream.message().await.unwrap() {
            if let Some(payload) = msg.payload {
                match payload {
                    protos::kaspad_response::Payload::GetBlockDagInfoResponse(resp_body) => {
                        println!("connected! GetBlockDagInfoResponse: {:?}", resp_body);
                    },
                    _ => {
                        anyhow::bail!("connect error")
                    }
                    
                }
            }
        }

        self.stream = Some(respone_stream);

        Ok(())
    }
}