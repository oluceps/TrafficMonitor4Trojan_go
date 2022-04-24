#![allow(unused)]

extern crate core;

use std::{process::Command, io, fs::File,string::String};
use std::io::Read;
use toml;
use serde_derive::Deserialize;
use toml::Value::String as tomlString;

#[derive(Deserialize)]
#[derive(Debug)]
struct User{
    hash:String,
    passwd:String,
    nick:String,
    status:UserStatus,
    upload_traffic:Option<i128>,
    download_traffic:Option<i128>,
    traffic_total:Option<i128>,
}
#[derive(Deserialize)]
#[derive(Debug)]
struct Server{
    address: Option<String>,
    port: u32,
}

#[derive(Deserialize)]
#[derive(Debug)]
enum UserStatus{
    Online,
    Offline{ last_login :String},
    NotExist,
}
#[derive(Deserialize)]
#[derive(Debug)]
struct Conf{
    server_info: Option<Vec<Server>>,
    user_info: Option<Vec<User>>
}

fn main() {
    handle_user_scale();
}


fn ls() {
    use std::string::String;
    let output = Command::new("ls")
        .output()
        .expect("Execute Error");

    let out = String::from_utf8(output.stdout).unwrap();

    println!("{}", out);
}

fn handle_user_scale() -> Option<u8>{
    use std::string::String;
    let file_path : &str = "/home/rito/Engineering/\
    TrafficMonitor4Trojan_go/config.toml";
    let mut config = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("config file: {} not found. exception: {}"
            ,file_path, e),

    };

    let mut config_cache = String::new();

    let n = match config.read_to_string(&mut config_cache) {
        Ok(s) => s,
        Err(e) => {panic!("error reading file ")},
    };
    Some(0)
}

//return the password of user
fn select_user<'a>() -> &'a str{
    use std::string::String;
    let mut usrinput = String::new();
    io::stdin().read_line(&mut usrinput)
        .expect("input arg error");
    let judge :u8 = usrinput.trim().parse().unwrap();
    "t"
}

