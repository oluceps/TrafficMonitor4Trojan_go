#![allow(unused)]
extern crate core;

use std::{process::Command, io, fs::File,string::String};
use std::alloc::handle_alloc_error;
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::io::Read;
use toml;
use serde_derive::Deserialize;
use toml::Value::String as tomlString;

#[derive(Deserialize)]
#[derive(Debug)]
struct User{

    hash: String,
    passwd: Option<String>,
    nick: Option<String>,
    //status:UserStatus,
    upload_traffic: Option<String>,
    download_traffic: Option<String>,
    traffic_total: Option<String>,

}


#[derive(Deserialize)]
#[derive(Debug)]
struct Server{
    address: Option<String>,
    manage_port: u32,
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
    address: String,
    manage_port: u32,
    users: HashMap<String,User>,
}

fn main() {
    handle_config_file();
    read_user_list_by_api();
}


fn read_user_list_by_api() {
    //./trojan-go -api-addr 127.0.0.1:10000 -api list | jq .[].status.user.hash
    let output = Command::new("ls")
        .output()
        .expect("Execute Error");

    let mut out = String::from_utf8(output.stdout).unwrap();
    let mut user_hash_list_raw = "\"1d6501dd05789331c94a765b0e1e2682d95ec06d8fd4c91703214b44\"
\"36417c02cab12df31a6c93c74b80a8bf485f90098c1b2ddb1a35367a\"
\"0e96722be6e675d749640634b35d0c6e6e7ee22232e927676afc2837\"".to_string().replace("\"","");

    let n = user_hash_list_raw.replace("\"","");

    let user_hash_list_from_api:Vec<&str> = n.split("\n").collect();

    //println!("{}",user_hash_list[0]);
    let i = handle_config_file().users;
    for per_user_hash in user_hash_list_from_api{
        for (user, inner) in i.iter(){
            if per_user_hash.to_string() == inner.hash{
                println!("TRUE  {:?} {:?}", per_user_hash, inner.hash);

            }
        }
    }




    //println!("{:#?}", i.iter());


}


fn handle_config_file() ->  Conf {
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
    //println!("{}",&config_cache);
    let config : Conf = toml::from_str(&config_cache).unwrap();

    config
}

