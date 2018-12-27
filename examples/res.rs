use r2ch::client::TwoCH;

fn main() {
    //TODO: normal naming
    let p = TwoCH::default()
        .board(Some("pr"))
        .thread(Some(1296509))
        .res();
    let r = p.unwrap();
    println!("{}", r);
}
