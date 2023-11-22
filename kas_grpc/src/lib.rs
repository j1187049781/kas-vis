pub mod client;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let mut c = client::KaspadClient::new(String::from("grpc://seeder2.kaspad.net:16110"));
        c.connect().await.unwrap();
        
    }
}
