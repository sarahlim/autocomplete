use std::io;

mod trie;
mod boggle;

fn main() {
    let t = trie::Node::from_file("./wikipedia-latest-titles-short.csv", 50000);

    loop {
        println!("\n🌟  Search for a title! 🌟");

        let mut query = String::new();

        io::stdin()
            .read_line(&mut query)
            .expect("🚨  Failed to read line 🚨");

        let results = t.autocomplete(&query.trim(), 10);

        println!("✨  Results ✨");
        for w in results {
            println!("✅  {}", w);
        }
    }
}
