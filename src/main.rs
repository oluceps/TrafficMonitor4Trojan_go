#![allow(unused)]

use std::{process::Command,io,fs::File};
use toml;
use serde_derive::Deserialize;

struct User{
    hash:str,
    passwd:str,
    nick:str,
    status:UserStatus,
    upload_traffic:Option<i128>,
    download_traffic:Option<i128>,
    traffic_total:Option<i128>,
}
enum UserStatus{
    Online,
    Offline{lastLogin: String},
    NotExist,
}

fn main() {
    ls();
}


fn ls() {
    let output = Command::new("ls")
        .output()
        .expect("Execute Error");

    let out = String::from_utf8(output.stdout).unwrap();

    println!("{}", out);
}

fn handle_user_scale() -> Option<u8>{
    let file_path : &str = "/home/rito/Engineering/\
    TrafficMonitor4Trojan_go/config.toml";
    let mut config = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("config file: {} not found. exception: {}"\
            ,file_path, e),
    };
}

//return the password of user
fn select_user() -> &String{
    let mut usrinput = String::new();
    io::stdin().read_line(&mut usrinput)
        .expect("input arg error");
    let judge :u8 = usrinput.trim().parse().unwrap();

}

