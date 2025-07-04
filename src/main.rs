use std::collections::HashMap;

mod indexer;

fn main() {
    let applications: HashMap<String, String> = indexer::get_applications();
    println!("{:?}", applications);
}
