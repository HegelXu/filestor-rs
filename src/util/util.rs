
use std::{io,fs};
use sha1::{Sha1, Digest};
use md5;
use base16ct;
use std::path::Path;



pub fn sha1(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    base16ct::lower::encode_string(&result)
   
}

pub fn file_sha1(file: &mut fs::File) ->String {
    let mut hasher = Sha1::new();
    io::copy( file, &mut hasher).unwrap();
    let result = hasher.finalize();
    base16ct::lower::encode_string(&result)
}

pub fn MD5(data: &[u8]) ->String {
    let res = md5::compute(data);
    format!("{:?}", res)
}

pub fn file_MD5(file: &mut fs::File) ->String {
    let mut ctx= md5::Context::new();
    io::copy(file, &mut ctx).unwrap();
    format!("{:?}",ctx.compute())
}

pub fn path_exists(path: String)-> bool{
    let target = Path::new(&path);
    target.exists()
}

pub fn get_file_size(filename: String) -> u64 {
    let target = Path::new(&filename);
    target.metadata().unwrap().len()
}