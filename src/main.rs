mod ui;
// use CommunicationCore::data_class::data_class::*;

use std::thread;
use CommunicationCore::net::{client::client, server::server};

fn main() {
    // ui::stater::stater(User::new("Vincent".to_string()));
    thread::spawn(|| {
        server::stater("127.0.0.1:6000");
    });

    client::stater("127.0.0.1:6000");
}
