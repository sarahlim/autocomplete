use std::io;
use std::io::{BufReader, BufRead};
use std::fs::File;

mod tries;

fn main() {
    let mut t = tries::Node::new();
    const FILES_TO_READ: i32 = 500000;

    let f = match File::open("./wikipedia-latest-titles-short.csv") {
        Ok(file) => file,
        Err(e) => panic!(e),
    };

    let file = BufReader::new(&f);
    let mut i = 0;

    for line in file.lines() {
        if i > FILES_TO_READ {
            break;
        }
        i += 1;
        t.insert(&line.unwrap());
    }

    loop {
        println!("\n🌟  Search for a title! 🌟");

        let mut query = String::new();

        io::stdin()
            .read_line(&mut query)
            .expect("🚨  Failed to read line 🚨");

        let results = t.autocomplete(&query.trim(), 10);

        println!("✨ Results ✨");
        for w in results {
            println!("✅  {}", w);
        }
    }
}
