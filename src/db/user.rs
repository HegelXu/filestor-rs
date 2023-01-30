use crate::db::mysql as mydb;
use mysql::prelude::*;
use mysql::*;

#[derive(Debug,Clone)]
pub struct User {
    pub Username: String,
    pub Email: String,
    pub Phone: String,
    pub SignupAt: String,
    pub LastActiveAt: String,
    pub Status: isize,
}

pub fn user_signup(username: String, passwd: String) -> bool {
    let binding = mydb::conn::DBConn();
    let mut conn = binding.lock().unwrap();
    conn.exec_drop(
        "insert ignore into tbl_user (`user_name`,`user_pwd`) values (:user_name,:user_pwd)",
        params! {
            "user_name" => username,
            "user_pwd" => passwd,
        },
    )
    .expect("insert error");

    if conn.affected_rows() > 0 {
        return true;
    }
    false
}

pub fn user_signin(username: String, encpwd: String) -> bool {
    let binding = mydb::conn::DBConn();
    let mut conn = binding.lock().unwrap();
    let query = format!(
        "select user_pwd from tbl_user where user_name={} limit 1",
        username
    );
    let res = conn
        .query_map(query, |pwd: String| pwd)
        .expect("query failed");

    if res.len() == 1 && res[0].eq(&encpwd) {
        return true;
    }

    false
}

pub fn update_token(username: String, token: String) -> bool {
    let binding = mydb::conn::DBConn();
    let mut conn = binding.lock().unwrap();
    conn.exec_drop(
        "replace into tbl_user_token (`user_name`,`user_token`) values (:user_name,:user_token)",
        params! {
            "user_name" => username,
            "user_token" => token,
        },
    )
    .expect("insert error");

    if conn.affected_rows() > 0 {
        return true;
    }
    false
}

pub fn get_user_info(username: String) -> Option<User> {
    let binding = mydb::conn::DBConn();
    let mut conn = binding.lock().unwrap();
    let query = format!(
        "select user_name,signup_at from tbl_user where user_name={} limit 1",
        username
    );
    let res = conn
        .query_map(
            query,
            |(username, email, phone, signupAt, lastActiveAt, status)| User {
                Username: username,
                Email: email,
                Phone: phone,
                SignupAt: signupAt,
                LastActiveAt: lastActiveAt,
                Status: status,
            },
        )
        .expect("query failed");

    if res.len() < 1 {
        return None;
    }
    Some(res[0].clone())
}

pub fn user_exist(username: String) -> Result<bool> {
    let binding = mydb::conn::DBConn();
    let mut conn = binding.lock().unwrap();
    let query = format!(
        "select Status from tbl_user where user_name={} limit 1",
        username
    );
    let res = conn
        .query_map(
            query,
            |status:usize| {
                status
            },
        )
        .expect("query failed");
    
    Ok(!res.is_empty())


}
