use r2ch::client::TwoCH;

fn main() {
    let data = TwoCH::default()
        .board(Some("pr"))
        .thread(Some(1296509))
        .res()
        .unwrap();
    println!("{}", data);
}
