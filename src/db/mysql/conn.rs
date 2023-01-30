use std::sync::Mutex;

use mysql::*;

use lazy_static::lazy_static;

use crate::config;

lazy_static! {
    #[derive(Clone)]
    pub static ref MYDB: Mutex<Conn> = {
        let url = config::db::MY_SQLSOURCE;
        // let pool = Pool::new(Opts::from_url(url).unwrap()).unwrap();

        // let mut conn: PooledConn = pool.get_conn().unwrap();
        // let res = conn.ping();
        // if !res {
        //     println!("Failed to connect to mysql");
        //     panic!("failed to connect to mysql");
        // }
        let mut conn = Conn::new(url).expect("Failed to connect to mysql");
        Mutex::new(conn)
        //Mutex::new(Conn::new(url).unwrap())


    };
}

pub(crate) fn DBConn() -> MYDB {
    MYDB.clone()
}

pub(crate) fn parse_rows(row: Row) {
    //-> HashMap<string, string> {
    //todo
    return;
}
