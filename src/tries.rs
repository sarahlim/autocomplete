#![allow(dead_code)]

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Node {
    value: Option<char>,
    is_word: bool,
    children: BTreeMap<char, Node>,
}

fn moving<T>(t: T) -> T {
    t
}

impl Node {
    pub fn new(value: Option<char>) -> Node {
        Node {
            value: value,
            is_word: false,
            children: BTreeMap::new(),
        }
    }

    /// Insert a word into the trie.
    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = moving(node)
                .children
                .entry(c)
                .or_insert(Node::new(Some(c)));
        }
        node.is_word = true;
    }

    /// Check whether the trie contains a word.
    pub fn has(&self, word: &str) -> bool {
        match self.find_node(&word) {
            Some(node) => node.is_word,
            None => false,
        }
    }

    /// Find the node corresponding to a given prefix.
    fn find_node(&self, prefix: &str) -> Option<&Node> {
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
            let mut current_word = prefix.to_owned();
            if let Some(current_char) = node.value {
                current_word.push(current_char);
            }
            if node.is_word {
                result.push(current_word.clone());
            }
            stack.extend(node.children
                             .values()
                             .rev()
                             .map(|child| (child, current_word.clone())));
        }
        result
    }

    /// Given a query and a number N, return the next N words
    /// prefixed with the query.
    pub fn autocomplete(&self, query: &str, n: usize) -> Vec<String> {
        if let Some(node) = self.find_node(query) {
            node.take_next(n)
                .iter()
                // HACK: Since take_next() appends each node's value
                // to the suffix, we accidentally double-count the
                // value of the node from which we begin searching.
                // Currently we just slice the suffix.
                .map(|suffix| format!("{}{}", query, &suffix[1..]))
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
        let mut t = Node::new(None);
        let words = vec!["aa", "ab", "abc", "Ab"];
        for w in words.iter() {
            t.insert(w);
        }
        for w in words.iter() {
            assert!(t.has(w));
            assert!(!t.has(&format!("{}{}", w, "x")));
            assert!(!t.has(&w[..1]));
        }
    }

    #[test]
    fn insert_one_letter() {
        // Inserting one-letter words works, without false positives.
        let mut t = Node::new(None);
        let words = vec!["aa", "ab", "c", "d"];
        for w in words.iter() {
            t.insert(w);
        }
        assert!(t.has("c"));
        assert!(t.has("d"));
        assert!(!t.has("a"));
    }

    #[test]
    fn insert_duplicate() {
        // Inserting duplicate words should only store one word.
        let mut t = Node::new(None);
        let words = vec!["aa", "aa"];
        for w in words.iter() {
            t.insert(w);
        }
        assert!(t.has("aa"));
        assert_eq!(t.autocomplete("a", words.len() + 1), vec!["aa"]);
    }

    #[test]
    fn autocomplete() {
        // Autocompletion works in lexicographic order.
        let mut t = Node::new(None);
        let words = vec!["abra", "aaron", "acapella", "z", "zzz", "zany"];
        for w in words.iter() {
            t.insert(w);
        }
        assert_eq!(t.autocomplete("a", words.len() + 1),
                   vec!["aaron", "abra", "acapella"]);
        assert_eq!(t.autocomplete("a", 1), vec!["aaron"]);
        assert_eq!(t.autocomplete("z", 2), vec!["z", "zany"]);
    }
}
