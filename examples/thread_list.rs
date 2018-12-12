use r2ch::client::TwoCH;

fn main() {
    let _ = TwoCH::default().board(Some("pr")).thread_list(Some(1));
}
