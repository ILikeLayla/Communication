mod ui;
use std::str;
use std::thread;
use std::rc::Rc;
use comm_core::{net::{client::client, server::server}, net_protocol::replacer::Replace, data_class::*};

fn main() {
    // ui::stater::stater(User::new("Vincent"));
    
    // thread::spawn(|| {
    //     server::stater("127.0.0.1:6000");
    // });

    // client::stater("127.0.0.1:6000");

    let from = Rc::new(User::new("from"));
    let to = Rc::new(User::new("to"));
    let test = Message::new(from, to, "hello!");

    let raw = test.to_rawdata();
    println!("{}", test);
    println!("{:?}", raw);
    println!("{}", str::from_utf8(&raw).unwrap());
    println!("{}", Message::from_rawdata(raw).unwrap());
}
