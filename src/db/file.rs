use crate::db::mysql as mydb;
use mysql::prelude::*;
use mysql::*;

pub fn on_file_upload_finished(
    filehash: String,
    filename: String,
    filesize: u64,
    fileaddr: String,
) -> bool {
    // let qery = format!("insert ignore into tbl_file (`file_sha1`,`file_name`,`file_size`,`file_addr`,`status`) values ({},{},{},{},1)",filehash,filename,filesize, fileaddr);
    // let stmt = mydb::conn::DBConn()
    //     .prep(qery.into())
    //     .expect("Failed to prepare statement");

    // let ret = stmt.params();
    let binding = mydb::conn::DBConn();
    let mut conn = binding.lock().unwrap();
    conn.exec_drop(
        r"insert ignore into tbl_file (`file_sha1`,`file_name`,`file_size`,`file_addr`,`status`) 
        values (:filehash,:filename,:filesize, :fileaddr,1)",
        params! {
            "filehash" => filehash,
            "filename" => filename,
            "filesize"=>filesize,
            "fileaddr"=>fileaddr,
        },
    ).expect("insert err");

    if conn.affected_rows() > 0 {
        return true;
    }

    false
}


#[derive(Debug,Clone)]
pub struct TableFile {
    pub FileHash: Option<String>,
    pub FileName: Option<String>,
    pub FileSize: Option<String>,
    pub FileAddr: Option<String>,
}

pub fn get_file_meta(filehash: String) -> Option<TableFile> {
    let binding = mydb::conn::DBConn();
    let mut conn = binding.lock().unwrap();
    let query  = format!("select file_sha1,file_addr,file_name,file_size from tbl_file where file_sha1={} and status=1 limit 1", filehash);
    let res = conn
        .query_map(query, |(file_sha1, file_addr, file_name, file_size)| {
            TableFile {
                FileHash: file_sha1,
                FileName: file_addr,
                FileSize: file_name,
                FileAddr: file_size,
            }
        })
        .expect("query failed");

    if res.len() < 1 {
        return None;
    }
    Some(res[0].clone())
}

//GetFileMetaList no use
pub fn upload_file_location(filehash: String, fileaddr: String) -> bool {
    let binding = mydb::conn::DBConn();
    let mut conn = binding.lock().unwrap();
    let res = conn.exec_drop(
        r"update tbl_file set`file_addr`=:fileaddr where  `file_sha1`=:filehash limit 1",
        params! {
            "filehash" => filehash,
            "fileaddr"=>fileaddr,
        },
    );
    if res.is_err() || conn.affected_rows()<1 {
        println!("update err");
        return false;
    }
    
    true
}
