use std::io::*;
use std::net::TcpStream;
use std::str::from_utf8;
use std::sync::{Mutex, Arc, Once, ONCE_INIT};
use std::{thread, time};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::Duration;

use console_chat::Console_chat;

pub struct Connection_manager {
    connection : Option<TcpStream>,
    is_running : bool,
}


impl Connection_manager {
    pub fn new() -> Connection_manager {
        Connection_manager {
            connection : None,
            is_running : true,
        }
    }

    pub fn command(&mut self, cmd: &[u8]) -> Option<String> {
        let cmd_str = from_utf8(cmd).unwrap();
        match cmd[0] as char {
            'q' => self.is_running = false,
            'c' => {
                match TcpStream::connect(&cmd_str[2 ..]) {
                    Ok(stream) => { 
                        match stream.set_read_timeout(Some(Duration::from_millis(500))) {
                            Ok(ok) => (),
                            Err(err) => return Some(format!("unable to set read timeout: {}", err)),
                        }
                        self.connection = Some(stream);
                    }
                    Err(err) => return Some(String::from("error connecting to server")),
                };
            },
            's' => {
                match self.connection {
                    None => return Some(String::from("not connected to a host")),
                    Some(ref mut con) => {
                        con.write(&cmd[2 ..]);
                    }
                }
            },
            _ => return Some(String::from("invalid command")),
        }
        None
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn read(&mut self, buf : &mut [u8]) -> Option<usize> {
        /*
        match self.connection {
            None => None,
            Some(ref mut con) => {
                let bytes = match con.read(buf);
                Some(bytes)
            },
        }
        */

        self.connection.as_ref().as_mut()?.read(buf).ok()
    }
}