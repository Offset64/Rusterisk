use super::*;

use std::io::{self, BufRead, Error, Read, Write};
use std::net::TcpStream;

pub struct AmiClient {
    reader: io::BufReader<TcpStream>,
    writer: io::BufWriter<TcpStream>,
}

impl AmiClient {
    pub fn new(addr: &str) -> Self {
        let conn = TcpStream::connect(addr).unwrap();

        AmiClient {
            reader: io::BufReader::new(conn.try_clone().unwrap()),
            writer: io::BufWriter::new(conn),
        }
    }

    pub fn login(&mut self, username: &str, secret: &str) {
        let msg = Message::Action(Login {
            username: username.to_owned(),
            secret: secret.to_owned(),
        });
        self.receive_message();
        self.send_message(msg);
        loop {
            self.receive_message();
        }
    }

    fn send_message<T: MessageDetails + std::fmt::Debug>(&mut self, msg: Message<T>) {
        log::trace!("Sending Message: {:?}", msg);
        match self.writer.write_all(&msg.as_bytes()) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn receive_message(&mut self) {
        log::trace!("Waiting for incoming message...");
        const BUFFER_SIZE: usize = 1024;
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut results = Vec::<u8>::new();
        loop {
            match self.reader.read(&mut buffer) {
                Ok(n) => match n {
                    1..=BUFFER_SIZE => {
                        log::trace!("[{}] recv - {:?}", n, &buffer);
                        results.extend_from_slice(&buffer);
                    }
                    BUFFER_SIZE => log::trace!("Not done yet"),
                    0 => {
                        log::trace!("got 0 back");
                        break;
                    }
                    _ => break,
                },
                Err(e) => {
                    todo!()
                }
            }
        }
        let readlen = results.len();
        // let readlen = &recv.len();
        log::trace!("Received {} bytes", readlen);
        log::trace!(
            "Received Message:\n{:#?}",
            String::from_utf8(results).unwrap()
        );
        // self.reader.consume(*readlen);
    }
}
