use anyhow;
use async_channel::Receiver;
use async_channel::Sender;
use protos::rpc_client::RpcClient;
use protos::KaspadRequest;
use protos::KaspadResponse;
use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use std::sync::Mutex;
use tonic::Streaming;

use self::protos::kaspad_request::Payload;

pub mod protos {
    tonic::include_proto!("protowire");
}

pub struct KaspadClient {
    req_sender: Sender<KaspadRequest>,
    req_receiver: Receiver<KaspadRequest>,
    resp_channle_map: Arc<Mutex<HashMap<u64, Sender<KaspadResponse>>>>,
    req_id: AtomicU64,
    host: String,
}

impl KaspadClient {
    pub fn new(host: String) -> Self {
        let (s, r) = async_channel::unbounded();
        return KaspadClient {
            req_sender: s,
            req_receiver: r,
            resp_channle_map: Arc::new(Mutex::new(HashMap::new())),
            req_id: AtomicU64::new(1),
            host,
        };
    }

    pub async fn get<T>(self: &mut Self, payload: Payload) -> Result<(), anyhow::Error> {
        let req_id = self.req_id.fetch_add(1, Relaxed);
        let msg: KaspadRequest = KaspadRequest {
            id: req_id,
            payload: Some(payload),
        };

        let (s, r) = async_channel::bounded(1);

        {
            let mut  m = self.resp_channle_map.lock().unwrap();
            m.insert(req_id, s);
        }

        self.req_sender.send(msg).await?;
        
        let ret = r.recv().await?;
        Ok(())
    }

    pub async fn connect(self: &Self) -> Result<(), anyhow::Error> {
        let mut client = RpcClient::connect(self.host.clone()).await.unwrap();

        let r = self.req_receiver.clone();
        let stream_request = async_stream::stream! {
            while let Ok(msg) = r.recv().await {
                yield msg;
            }
        };

        let request = tonic::Request::new(stream_request);
        let respone_stream = client.message_stream(request).await.unwrap();

        let respone_stream = respone_stream.into_inner();
        // let resp_sender_map = self.resp_channle_map;
        // tokio::spawn(async move {
        //     Self::handle_resp_loop(& resp_sender_map, respone_stream);
        // });

        Ok(())
    }

    async fn handle_resp_loop(
        resp_sender_map: & Arc<Mutex<HashMap<u64, Sender<KaspadResponse>>>>,
        mut stream: Streaming<KaspadResponse>,
    ) {
        while let Some(msg) = stream.message().await.unwrap() {
            // resp_sender.send(msg).await.unwrap();
        }
    }
}
