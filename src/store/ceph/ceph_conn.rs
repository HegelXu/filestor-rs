use crate::config as cfg;
use once_cell::sync::OnceCell;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::region::Region;
use s3::request_trait::ResponseData;

// lazy_static! {
//     #[derive(Clone)]
//     pub static ref CREDENTIALS: Credentials = {
//         let access_key = Some(cfg::ceph::CEPH_ACCESS_KEY);
//         let secret_key = Some(cfg::ceph::CEPH_SECRET_KEY);
//         let credentials = Credentials::new(
//             access_key,
//             secret_key,
//             None,
//             None,
//             None,
//         );
//         credentials.unwrap()
//     };
// }

// const CREDENTIALS: Credentials = Credentials::new(
//     Some(cfg::ceph::CEPH_ACCESS_KEY),
//     Some(cfg::ceph::CEPH_SECRET_KEY),
//     None,
//     None,
//     None,
// )
// .unwrap();
static CREDENTIALS: OnceCell<Credentials> = OnceCell::new();
static CURREGION: OnceCell<Region> = OnceCell::new();
//cfg::ceph::REGION.parse().unwrap();

// lazy_static! {
//     #[derive(Clone)]
//     pub static ref CURREGION: Region = {
//         cfg::ceph::REGION.parse().unwrap()
//     };
// }

pub fn get_ceph_connection() -> (Credentials, Region) {
    let cred = CREDENTIALS.get_or_init(|| {
        let access_key = Some(cfg::ceph::CEPH_ACCESS_KEY);
        let secret_key = Some(cfg::ceph::CEPH_SECRET_KEY);
        let credentials = Credentials::new(access_key, secret_key, None, None, None);
        credentials.unwrap()
    });

    let rgn = CURREGION.get_or_init(|| cfg::ceph::REGION.parse().unwrap());

    return (cred.clone(), rgn.clone());
}

pub fn get_ceph_bucket(bucket: String) -> Bucket {
    let (cred, rgn) = get_ceph_connection();
    let ret = Bucket::new(&bucket, rgn, cred).expect("get bucket error");
    ret
}

pub async fn pub_object(path:String, content:&[u8]) -> Result<ResponseData, S3Error>{
    get_ceph_bucket("bucket".into()).put_object(path, content).await
}