use crate::config::aws_s3 as cfg;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::presigning;
use aws_sdk_s3::{operation::PutObject, Client};
use std::time::Duration;
//use once_cell::sync::OnceCell;

//static S3CLIENT: OnceCell<Client> = OnceCell::new();

pub async fn client() -> Client {
    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    client
}

pub async fn bucket() -> aws_sdk_s3::client::fluent_builders::PutObject {
    let client = client().await;
    let bucket = client.put_object().bucket(cfg::BUCKET);
    bucket
}

pub async fn download_URL(objName: String) -> String {
    let cli = client().await;
    let presigning_config = presigning::config::PresigningConfig::builder()
        .expires_in(Duration::from_secs(3600))
        .build()
        .unwrap();
    let req = cli
        .get_object()
        .bucket(cfg::BUCKET)
        .key(cfg::ACCESS_KEY_SECRET);
    let url = req.presigned(presigning_config).await.unwrap();
    url.uri().to_string()
}
