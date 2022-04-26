

pub mod config_file_def{

    use serde_derive::{Deserialize,Serialize};
    use std::collections::HashMap;

    #[derive(Deserialize,Serialize,Debug)]
    pub struct User{

        pub hash: String,
        pub passwd: Option<String>,
        pub nick: Option<String>,
        //status:UserStatus,
        pub upload_traffic: Option<String>,
        pub download_traffic: Option<String>,
        pub traffic_total: Option<String>,

    }


    #[derive(Deserialize,Serialize,Debug)]
    pub struct Server{
        pub address: Option<String>,
        pub manage_port: u32,
    }

    #[derive(Deserialize,Serialize,Debug)]
    pub enum UserStatus{
        Online,
        Offline{ last_login :String},
        NotExist,
    }



    #[derive(Deserialize,Serialize,Debug)]
    pub struct Conf{
        pub address: String,
        pub manage_port: u32,
        pub users: HashMap<String,User>,
    }

}