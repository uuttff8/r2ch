// Third Party
use reqwest::Error;

#[derive(Debug, Clone)]
pub struct TwoCH<'a> {
    prefix: String,
    board: Option<&'a str>,
    thread: Option<u32>,
}

impl<'a> TwoCH<'a> {
    // Public methods
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
    pub fn res(&self) -> Result<(), Box<Error>> {
        let url = format!(
            "{}{}/res/{}.json",
            self.prefix,
            self.board.unwrap(),
            self.thread.unwrap()
        );
        self.get(url)
    }

    // It's another value of threads (i.e. 1, 2, 3)
    pub fn thread_list(&self, thread: Option<u32>) -> Result<(), Box<Error>> {
        let url = format!(
            "{}{}/{}.json",
            self.prefix,
            self.board.unwrap(),
            thread.unwrap()
        );
        self.get(url)
    }

    // RU: Все треды с сортировкой по последнему посту:
    // EN: All thread with iter by last post
    // http(s)://2ch.hk/доска/catalog.json
    pub fn catalog(&self) -> Result<(), Box<Error>> {
        self.get_catalog("catalog")
    }

    // https://2ch.hk/доска/catalog_num.json
    pub fn catalog_num(&self) -> Result<(), Box<Error>> {
        self.get_catalog("catalog_num")
    }

    // https://2ch.hk/makaba/mobile.fcgi?task=get_boards
    pub fn boards_all(&self) -> Result<(), Box<Error>> {
        let url = format!("{}makaba/mobile.fcgi?task=get_boards", self.prefix,);
        self.get(url)
    }

    // https://2ch.hk/makaba/mobile.fcgi?task=get_thread&board=abu&thread=39220&num=41955
    pub fn posts_by_board(&self, num: u32) -> Result<(), Box<Error>> {
        let url = format!(
            "{}makaba/mobile.fcgi?task=get_thread&board={}&thread={}&num={}",
            self.prefix,
            self.board.unwrap(),
            self.thread.unwrap(),
            num
        );
        self.get(url)
    }

    // https://2ch.hk/makaba/mobile.fcgi?task=get_thread&board=abu&thread=39220&post=252
    pub fn posts_by_thread(&self, post: u32) -> Result<(), Box<Error>> {
        let url = format!(
            "{}makaba/mobile.fcgi?task=get_thread&board={}&thread={}&post={}",
            self.board.unwrap(),
            self.thread.unwrap(),
            self.prefix,
            post
        );
        self.get(url)
    }

    pub fn post_by_thread(&self, post: u32) -> Result<(), Box<Error>> {
        let url = format!(
            "{}makaba/mobile.fcgi?task=get_post&board={}&post={}",
            self.prefix,
            self.board.unwrap(),
            post
        );
        self.get(url)
    }

    // Private methods
    fn get_catalog(&self, access: &'a str) -> Result<(), Box<Error>> {
        let url = format!("{}{}/{}.json", self.prefix, self.board.unwrap(), access);
        self.get(url)
    }

    fn get(&self, url: String) -> Result<(), Box<Error>> {
        let link = url.as_str();
        let body = reqwest::get(link)?.text()?;
        println!("{}", body);
        Ok(())
    }
}
