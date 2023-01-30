use serde::{ Deserialize, Serialize };
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RespMsg {
    pub Code: Option<isize>,
    pub Msg: Option<String>,
    pub Data: Option<serde_json::Value>,
}

pub fn new_respmsg(code:isize, msg:String, data:serde_json::Value)->RespMsg {
    let resp = RespMsg {
        Code:Some(code),
        Msg:Some(msg),
        Data:Some(data),
    };
    resp
}

impl RespMsg {
    pub fn JSONBytes(& self) -> Vec<u8> {
        let binding = serde_json::to_vec(&self).unwrap();
        
        binding
    }

    pub fn JSONString(&self) -> String {
        let binding = serde_json::to_string(&self).unwrap();

        binding
        
    }

}
