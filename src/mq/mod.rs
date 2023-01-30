
pub mod consumer;
pub mod producer;

use crate::common as cmn;
pub struct TransferData  {
    FileHash      :String,
	CurLocation   :String,
	DestLocation  :String,
	DestStoreType :cmn::store::StoreType,
} 