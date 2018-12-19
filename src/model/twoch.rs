pub struct Thread {
    pub name: String,
    pub desc: String,
}

pub struct Message {
    pub name: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct TwoCH<'a> {
    prefix: String,
    board: Option<&'a str>,
    thread: Option<u32>,
}