use rusterisk::AmiClient;
use simple_logger::SimpleLogger;

// use rusterisk::message::{Login, Message, ShowEndpoints};

fn main() {
    SimpleLogger::new().init().unwrap();

    let mut client = AmiClient::new("192.168.1.32:5038");
    client.login("example_ami", "2f35413cc35f27e96b1cd19c6930e4a4");
}
