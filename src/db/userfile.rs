use crate::db::mysql as mydb;
use chrono::prelude::*;
use mysql::prelude::*;
use mysql::*;

pub struct UserFile {
    pub UserName: String,
    pub FileHash: String,
    pub FileName: String,
    pub FileSize: i64,
    pub UploadAt: String,
    pub LastUpdated: String,
}

pub fn on_user_file_upload_finished(
    username: String,
    filehash: String,
    filename: String,
    filesize: i64,
) -> bool {
    let binding = mydb::conn::DBConn();
    let mut conn = binding.lock().unwrap();
    conn.exec_drop(
        "insert ignore into tbl_user_file (`user_name`,`file_sha1`,`file_name`, `file_size`,`upload_at`) values (:username, :filehash, :ilename, :filesize, :time)",
        params! {
            "username" => username,
            "filehash" => filehash,
            "filename" =>filename,
            "filesize" =>filesize,
            "time" =>  Local::now().to_string(),
        },
    )
    .expect("insert error");

    if conn.affected_rows() > 0 {
        return true;
    }
    false
}

pub fn query_user_file_metas(username: String, limit: usize) -> Option<Vec<UserFile>> {
    let binding = mydb::conn::DBConn();
    let mut conn = binding.lock().unwrap();
    let query = format!(
        "select file_sha1,file_name,file_size,upload_at,last_update from tbl_user_file where user_name={} limit {}",
        username,limit
    );
    let res = conn
        .query_map(
            query,
            |(username, fileHash, fileName, fileSize, uploadAt, uastUpdated)| UserFile {
                UserName: username,
                FileHash: fileHash,
                FileName: fileName,
                FileSize: fileSize,
                UploadAt: uploadAt,
                LastUpdated: uastUpdated,
            },
        )
        .expect("query failed");

    if res.len() < 1 {
        return None;
    }
    Some(res)
}

pub fn delete_user_file(username: String, filehash: String) -> bool {
    let binding = mydb::conn::DBConn();
    let mut conn = binding.lock().unwrap();
    conn.exec_drop(
        "update tbl_user_file set status=2 where user_name=:username and file_sha1=:filehash limit 1",
        params! {
            "username" => username,
            "filehash" => filehash,
        },
    )
    .expect("insert error");

    if conn.affected_rows() > 0 {
        return true;
    }
    false
}

pub fn rename_file_name(username: String, filehash: String, filename: String) -> bool {
    let binding = mydb::conn::DBConn();
    let mut conn = binding.lock().unwrap();
    conn.exec_drop(
        "update tbl_user_file set file_name=:filename where user_name=:username and file_sha1=:filehash limit 1",
        params! {
            "username" => username,
            "filehash" => filehash,
            "filename" => filename,
        },
    )
    .expect("insert error");

    if conn.affected_rows() > 0 {
        return true;
    }
    false
}

pub fn query_user_filemeta(username: String, filehash: String) -> Option<UserFile> {
    let binding = mydb::conn::DBConn();
    let mut conn = binding.lock().unwrap();
    let query = format!(
        "select file_sha1,file_name,file_size,upload_at,last_update from tbl_user_file where user_name={} and file_sha1={} limit 1",
        username,filehash
    );
    let mut res = conn
        .query_map(
            query,
            |(username, fileHash, fileName, fileSize, uploadAt, uastUpdated)| UserFile {
                UserName: username,
                FileHash: fileHash,
                FileName: fileName,
                FileSize: fileSize,
                UploadAt: uploadAt,
                LastUpdated: uastUpdated,
            },
        )
        .expect("query failed");
        
    res.pop()
}
