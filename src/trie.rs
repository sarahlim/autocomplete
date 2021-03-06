#![allow(dead_code)]

use std::collections::BTreeMap;
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug)]
pub struct Node {
    is_word: bool,
    children: BTreeMap<char, Node>,
}

fn moving<T>(t: T) -> T {
    t
}

impl Node {
    pub fn new() -> Node {
        Node {
            is_word: false,
            children: BTreeMap::new(),
        }
    }

    pub fn from_file(path: &str, max_lines: usize) -> Node {
        let mut t = Node::new();
        let f = match File::open(path) {
            Ok(file) => file,
            Err(e) => panic!(e),
        };
        let file = BufReader::new(&f);

        for (i, line) in file.lines().enumerate() {
            if i > max_lines {
                break;
            }
            t.insert(&line.unwrap());
        }
        t
    }

    pub fn is_word(&self) -> bool {
        self.is_word
    }

    /// Insert a word into the trie.
    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = moving(node).children.entry(c).or_insert(Node::new());
        }
        node.is_word = true;
    }

    /// Check whether the trie contains a word.
    pub fn has_word(&self, word: &str) -> bool {
        match self.find_node(&word) {
            Some(node) => node.is_word,
            None => false,
        }
    }

    /// Find the node corresponding to a given prefix.
    pub fn find_node(&self, prefix: &str) -> Option<&Node> {
        let mut node = self;
        for c in prefix.chars() {
            node = match node.children.get(&c) {
                Some(child) => child,
                None => return None,
            }
        }
        Some(node)
    }

    /// Get the next N words from the current node.
    // TODO: Refactor this using an iterator.
    pub fn take_next(&self, n: usize) -> Vec<String> {
        let mut stack = vec![(self, String::new())];
        let mut result = vec![];

        while let Some((node, prefix)) = stack.pop() {
            if result.len() >= n {
                return result;
            }
            if node.is_word {
                result.push(prefix.clone());
            }
            stack.extend(node.children
                             .iter()
                             .rev()
                             .map(|(edge, child)| (child, format!("{}{}", prefix, edge))));
        }
        result
    }

    /// Given a query and a number N, return the next N words
    /// prefixed with the query.
    pub fn autocomplete(&self, query: &str, n: usize) -> Vec<String> {
        if let Some(node) = self.find_node(query) {
            node.take_next(n)
                .iter()
                .map(|suffix| format!("{}{}", query, &suffix))
                .collect::<Vec<String>>()
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        // Basic insertion and lookup
        let mut t = Node::new();
        let words = vec!["aa", "ab", "abc", "Ab"];
        for w in words.iter() {
            t.insert(w);
        }
        for w in words.iter() {
            assert!(t.has_word(w));
            assert!(!t.has_word(&format!("{}{}", w, "x")));
            assert!(!t.has_word(&w[..1]));
        }
    }

    #[test]
    fn insert_one_letter() {
        // Inserting one-letter words works, without false positives.
        let mut t = Node::new();
        let words = vec!["aa", "ab", "c", "d"];
        for w in words.iter() {
            t.insert(w);
        }
        assert!(t.has_word("c"));
        assert!(t.has_word("d"));
        assert!(!t.has_word("a"));
    }

    #[test]
    fn insert_duplicate() {
        // Inserting duplicate words should only store one word.
        let mut t = Node::new();
        let words = vec!["aa", "aa"];
        for w in words.iter() {
            t.insert(w);
        }
        assert!(t.has_word("aa"));
        assert_eq!(t.autocomplete("a", words.len() + 1), vec!["aa"]);
    }

    #[test]
    fn autocomplete() {
        // Autocompletion works in lexicographic order.
        let mut t = Node::new();
        let words = vec!["abra", "aaron", "acapella", "z", "zzz", "zany"];
        for w in words.iter() {
            t.insert(w);
        }
        assert_eq!(t.autocomplete("a", words.len() + 1),
                   vec!["aaron", "abra", "acapella"]);
        assert_eq!(t.autocomplete("a", 1), vec!["aaron"]);
        assert_eq!(t.autocomplete("z", 2), vec!["z", "zany"]);
    }

    #[test]
    fn init_from_file() {
        // Can initialize a trie from a file of words.
        let t = Node::from_file("./wikipedia-latest-titles-short.csv", 10);
        assert!(t.has_word("Synontology"));
        assert!(t.has_word("Prince Regent gudgeon"));
        assert!(t.has_word("Reported Military Losses during the Invasion of Cyprus (1974)"));
        assert!(!t.has_word("RU-38"));
    }
}
