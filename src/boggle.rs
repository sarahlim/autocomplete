#![allow(dead_code)]
extern crate bit_vec;

use std::collections::HashSet;
use self::bit_vec::BitVec;
use super::trie;

// Super basic Boggle solver, using tries.

// TODO: Make this take a reference to a Grid.
pub struct Boggle {
    board: Vec<Vec<char>>,
    n: usize,
    lexicon: trie::Node,
}

struct Posn {
    x: usize,
    y: usize,
}

impl Boggle {
    fn new(board: Grid, lexicon_path: &str) -> Boggle {
        Boggle {
            board: board.data,
            n: board.n,
            lexicon: trie::Node::from_file(lexicon_path, 20000),
        }
    }

    /// Find all the words in a Boggle board.
    pub fn solve(&self) -> HashSet<String> {
        let n = self.n;
        let mut found = HashSet::new();
        for x in 0..n {
            for y in 0..n {
                let visited = Visited::new(n);
                self.visit_recur(Posn { x: x, y: y }, "", visited, &mut found);
            }
        }
        found
    }

    fn visit_recur(&self, posn: Posn, acc: &str, visited: Visited, found: &mut HashSet<String>) {
        // Check word at posn.
        let Posn { x, y } = posn;
        let current = self.board[x][y];
        let mut word = String::from(acc);
        word.push(current);
        if let Some(node) = self.lexicon.find_node(&word) {
            // Check if the node is a word, or just a valid prefix.
            if node.is_word() {
                found.insert(word.clone());
            }
            // Recursively search neighbors.
            let neighbors = self.unvisited_neighbors(&posn, &visited);
            for neighbor in neighbors {
                let mut next_visited = visited.clone();
                next_visited.mark_visited(&posn);
                self.visit_recur(neighbor, &word, next_visited, found);
            }
        }
    }

    /// Generate all the adjacent Posns for a given Posn.
    fn neighbors(&self, posn: &Posn) -> Vec<Posn> {
        let mut result = vec![];
        let &Posn { x, y } = posn;
        if x > 0 {
            result.push(Posn { x: x - 1, y: y });
            if y > 0 {
                result.push(Posn { x: x - 1, y: y - 1 });
            }
            if y < self.n - 1 {
                result.push(Posn { x: x - 1, y: y + 1 });
            }
        }
        if x < self.n - 1 {
            result.push(Posn { x: x + 1, y: y });
            if y < self.n - 1 {
                result.push(Posn { x: x + 1, y: y + 1 });
            }
            if y > 0 {
                result.push(Posn { x: x + 1, y: y - 1 });
            }
        }
        if y > 0 {
            result.push(Posn { x: x, y: y - 1 });
        }
        if y < self.n - 1 {
            result.push(Posn { x: x, y: y + 1 });
        }
        result
    }

    /// Generate all unvisited adjacent Posns for a given Posn.
    fn unvisited_neighbors(&self, posn: &Posn, visited: &Visited) -> Vec<Posn> {
        self.neighbors(posn)
            .into_iter()
            .filter(|posn| !visited.check(posn))
            .collect()
    }
}

#[derive(Clone)]
struct Visited {
    data: BitVec,
    n: usize,
}

impl Visited {
    fn new(n: usize) -> Visited {
        Visited {
            n: n,
            data: bit_vec::BitVec::from_elem(n * n, false),
        }
    }

    fn mark_visited(&mut self, posn: &Posn) {
        let i = self.index(posn);
        self.data.set(i, true);
    }

    fn check(&self, posn: &Posn) -> bool {
        let i = self.index(&posn);
        self.data[i]
    }

    fn index(&self, posn: &Posn) -> usize {
        posn.y * self.n + posn.x
    }
}

struct Grid {
    data: Vec<Vec<char>>,
    n: usize,
}

impl Grid {
    fn from_data(data: &str, n: usize) -> Grid {
        if data.len() != n * n {
            panic!("Data strings must be N * N in length");
        }
        Grid {
            n: n,
            data: data.to_lowercase()
                .chars()
                .collect::<Vec<_>>()
                .chunks(n)
                .map(|row| row.iter().cloned().collect())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_board(data: &str, n: usize, expected: &HashSet<&str>) {
        let grid = Grid::from_data(data, n);
        let boggle = Boggle::new(grid, "./bogwords.txt");
        let result = boggle.solve();
        for word in expected.iter() {
            // HACK: Not sure how to check a HashSet of Strings for
            // an &str without turning the &str into a String...
            assert!(result.contains(word.to_owned()),
                    format!("Missing word: {}", word));
        }
    }

    #[test]
    fn small_board() {
        let expected: HashSet<&str> = ["urn", "seat", "sea", "eat", "ton"]
            .iter()
            .cloned()
            .collect();
        test_board("xqaezotsindlyruk", 4, &expected);
    }

    #[test]
    fn large_board() {
        let expected: HashSet<&str> = {
            let words = ["yokel", "sake", "fee", "ami", "cue", "sky", "lea", "you", "amy", "foam",
                         "sax", "cow", "seamy", "woe", "easy", "yak", "sexy", "amok", "mix",
                         "sec", "sea", "oak", "seam", "max", "make", "keys", "may", "few", "leak",
                         "key", "aim", "say", "fox", "ask", "sex", "mask", "foe", "fum", "yea",
                         "yoke", "tau", "wee", "foamy", "okay", "leaky", "owe", "yam"];
            words.iter().cloned().collect()
        };
        test_board("taiakhuwiyxkuxmasyeuokexcefyclqowywh", 6, &expected);
    }
}
