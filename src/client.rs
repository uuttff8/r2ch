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

    // Use this only as in example [res.rs]
    pub fn build_res(&self) -> Result<(), Box<Error>> {
        let url = format!(
            "{}{}/res/{}.json",
            self.prefix,
            self.board.unwrap(),
            self.thread.unwrap()
        );
        let link = url.as_str();
        let body = reqwest::get(link)?.text()?;
        println!("{}", body);
        Ok(())
    }

    // It's another value of threads (i.e. 1, 2, 3)
    pub fn build_simple(&self, thread: Option<u32>) -> Result<(), Box<Error>> {
                let url = format!(
            "{}{}/{}.json",
            self.prefix,
            self.board.unwrap(),
            thread.unwrap()
        );
        let link = url.as_str();
        let body = reqwest::get(link)?.text()?;
        println!("{}", body);
        Ok(())
    }

    // RU: Все треды с сортировкой по последнему посту:
    // EN: All thread with iter by last post
    // http(s)://2ch.hk/доска/catalog.json
    pub fn catalog(&self) -> Result<(), Box<Error>> {
        self.get_catalog(Some("catalog"));
        Ok(())
    }

    // https://2ch.hk/доска/catalog_num.json
    pub fn catalog_num(&self) -> Result<(), Box<Error>> {
        self.get_catalog(Some("catalog_num"));
        Ok(())
    }

    // private methods
    fn get_catalog(&self, access: Option<&'a str>) -> Result<(), Box<Error>> {
        let url = format!(
            "{}{}/{}.json",
            self.prefix,
            self.board.unwrap(),
            access.unwrap()
        );
        let link = url.as_str();
        let body = reqwest::get(link)?.text()?;
        println!("{}", body);
        Ok(())
    }
}
