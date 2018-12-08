// Third Party
use reqwest::Error;

#[derive(Debug, Clone)]
pub struct TwoCH<'a> {
    prefix: String,
    board: Option<&'a str>,
    thread: Option<u32>,
}

impl<'a> TwoCH<'a> {
    // public methods

    pub fn default() -> TwoCH<'a> {
        TwoCH {
            prefix: String::from("https://2ch.hk/"),
            board: None,
            thread: None,
        }
    }

    pub fn board(mut self, board: Option<&'a str>) -> TwoCH<'a> {
        self.board = board;
        self
    }

    pub fn thread(mut self, thread: Option<u32>) -> TwoCH<'a> {
        self.thread = thread;
        self
    }

    pub fn build(&self) -> Result<(), Box<Error>> {
        let url = format!("{}{}/res/{}.json", self.prefix, self.board.unwrap(), self.thread.unwrap());
        let link = url.as_str();
        let body = reqwest::get(link)?.text()?;
        println!("body = {}", body);
        Ok(())
    }
}