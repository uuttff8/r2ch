use crate::client::TwoCH;

pub struct Json;

impl Json {
    pub fn post(&self) { 
        /* TwoCH::default()
            .board(TwoCH.board.unwrap())
            .thread(TwoCH.thread.unwrap());
        */

        unimplemented!();
    }

    pub fn thread(&self) {
        unimplemented!();
    }

    /// param
    /// @param: TwoCH.prefix (optional)
    /// @param: TwoCH.board (required)
    pub fn settings_captha_thread(&self) {
        /* format!(
            "{}/api/captcha/settings/{}",
            TwoCH.prefix,
            TwoCH.board
        ); */
        unimplemented!();
    }
}