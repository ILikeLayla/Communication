mod ui;
use std::str;
use std::thread;
use comm_core::{net::{client::client, server::server}, net_protocol::Replace, data_class::*, };

fn main() {
    // ui::stater::stater(User::new("Vincent".to_string()));
    // thread::spawn(|| {
    //     server::stater("127.0.0.1:6000");
    // });

    // client::stater("127.0.0.1:6000");
    let time = Local::now();
    let raw = time.to_rawdata();
    println!("{}", str::from_utf8(&raw).unwrap());
    println!("{:?}", raw);
    println!("{}", DateTime::<Local>::from_rawdata(raw).unwrap())
}
