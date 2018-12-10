use r2ch::client::TwoCH;

fn main() {
    let _ = TwoCH::default()
        .board(Some("pr"))
        .catalog_num();
}