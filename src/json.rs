use serde_json::{Error, Value};

use crate::client::TwoCH;

impl<'a> TwoCH<'a> {
    pub fn res_json(&self) -> Result<(), Error> {
        let data = TwoCH::default().thread(self.thread).board(self.board).res();
        // TODO: make normal naming of lets
        let s = data.unwrap();
        let p: Value = serde_json::from_str(&s).unwrap();

        println!("{}", p["Board"]);
        Ok(())
    }

    pub fn settings_captha_thread(&self) {
        /* format!(
            "{}/api/captcha/settings/{}",
            TwoCH.prefix,
            TwoCH.board
        ); */
        unimplemented!();
    }
}
