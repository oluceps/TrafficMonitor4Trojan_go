#![allow(unused)]

extern crate core;

use traffic_monitor4_trojan_go::config_struct::config_file_def::*;
use std::{process::Command, io, fs::File,string::String};
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::io::Read;
use toml;

use toml::Value::String as tomlString;

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

    let mut user_hash_list_from_api:Vec<&str> = n.split("\n").collect();

    //println!("{}",user_hash_list[0]);
    let i = handle_config_file().users;

    let mut vec_user_hash: Vec<String> = Vec::new();


    for (user,inner) in i.iter(){
        vec_user_hash.push(inner.hash.to_string())
    }

    for per_user_hash_from_api in user_hash_list_from_api.iter(){
        if vec_user_hash.contains(&per_user_hash_from_api.to_string()){
            println!("api_person in file : {:?}", per_user_hash_from_api);
        }
    }
    let mut file_not_contain:Vec<&str>;

    for per_usr_from_file in vec_user_hash.iter(){
        let processed: &str = per_usr_from_file.as_str();
        if !user_hash_list_from_api.contains(&processed){
            println!("file person not in api : {:?}", processed);
        }
    }
    //println!("read from file uncontain load from api: {:?}", aa);





}
fn load_config_file() -> File {
    use std::string::String;
    let file_path : &str = "/home/rito/Engineering/\
    TrafficMonitor4Trojan_go/config.toml";

    let config = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("config file: {} not found. exception: {}"
                         ,file_path, e),
    };
    config
}


enum UserAd{
    Add,
    Del,
}

fn handle_user<T>(stats: &T){
    match stats {
        &_ => {}
    }
    let config = load_config_file();
    let mut to_add = String::new();

}

fn handle_config_file() ->  Conf {

    let mut config = load_config_file();

    let mut config_cache = String::new();

    let n = match config.read_to_string(&mut config_cache) {
        Ok(s) => s,
        Err(e) => {panic!("error reading file ")},
    };
    //println!("{}",&config_cache);
    let config : Conf = toml::from_str(&config_cache).unwrap();

    config
}

