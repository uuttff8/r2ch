extern crate serde_json;
extern crate reqwest;

use std::fmt;
use std::option;
use std::error;

#[derive(Debug, Clone)]
pub struct TwoCH {
    prefix: String,
    board: Option<u8>,
    thread: Option<u8>,
}

impl TwoCH {
    pub fn new(&self, board: Option<u8>, thread: Option<u8>) -> TwoCH {
        TwoCH {
            prefix: "https://2ch.hk/".into(),
            board,
            thread,
        }
    }

    pub fn get_thread(&self, board: Option<u8>, thread: Option<u8>) -> Result<TwoCH, Box<error::Error>> {
        let link = format!("{}{}/{}/", self.prefix, self.board, self.thread).into();
        let body = reqwest::get(link)?
            .text()?;
        println!("{}", body);
        Ok(())
    }
}


impl fmt::Display for TwoCH {
    format!("");
}