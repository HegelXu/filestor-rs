use crate::common;

pub const TEMP_LOCAL_ROOT_DIR: &str = "/data/fileserver/";
pub const TEMP_PART_ROOT_DIR: &str = "/data/fileserver_part/";
pub const CEPH_ROOT_DIR: &str = "/ceph";
pub const OSSROOT_DIR: &str = "oss/";
//pub const CURRENT_STORE_TYPE: &str = common::store::StoreType::StoreLocal;
pub const CURRENT_STORE_TYPE:common::store::StoreType = common::store::StoreType::StoreLocal;
