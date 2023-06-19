mod ui;
use base::data_class::data_class::*;

use std::thread;
use base::net::{client::Client, server::Server};

fn main() {
    // ui::stater::stater(User::new("Vincent".to_string()));
    thread::spawn(|| {
        Server::server("127.0.0.1:6000");
    });

    Client::client::<Vec<u8>>("127.0.0.1:6000");
}
