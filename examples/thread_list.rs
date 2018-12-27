use r2ch::client::TwoCH;

fn main() {
    let _ = TwoCH::default()
        .board(Some("pr"))
        .thread(Some(1))
        .thread_list();
}
