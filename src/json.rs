use serde_json::{Error, Value};

use crate::client::TwoCH;

impl<'a> TwoCH<'a> {
    pub fn res_json(&self) -> Result<(), Error> {
        let data = TwoCH::default()
            .thread(self.thread)
            .board(self.board)
            .res()
            .unwrap();
        let json: Value = serde_json::from_str(&data).unwrap();
        println!("Board: {}", json["Board"]);
        println!("BoardName: {}", json["BoardName"]);
        Ok(())
    }
}
