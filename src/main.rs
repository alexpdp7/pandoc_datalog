fn main() {
    let d: pandoc_types::definition::Pandoc = serde_json::from_reader(std::io::stdin()).unwrap();
    for block in d.blocks {
        println!("̣{block:?}");
    }
}
