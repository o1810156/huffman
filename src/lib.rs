extern crate bimap;

pub mod huffman {
    use std::cmp::Ordering;
    // use std::collections::HashMap;
    use bimap::BiMap;
    use std::fmt;
    use std::fmt::Display;
    use std::hash::Hash;

    enum Node<T>
    where
        T: Display + PartialEq + Eq + Hash + Clone,
    {
        Internal {
            freq: f64,
            left: Box<Node<T>>,
            right: Box<Node<T>>,
        },
        Leaf {
            freq: f64,
            label: T,
        },
    }

    impl<T> Display for Node<T>
    where
        T: Display + PartialEq + Eq + Hash + Clone,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Node::Internal { freq, left, right } => {
                    write!(f, "{:.3} {{{}, {}}}", freq, left, right)
                }
                Node::Leaf { freq, label } => write!(f, "{:.3} {}", freq, label),
            }
        }
    }

    impl<T> Ord for Node<T>
    where
        T: Display + PartialEq + Eq + Hash + Clone,
    {
        fn cmp(&self, other: &Self) -> Ordering {
            self.get_freq()
                .partial_cmp(&other.get_freq())
                .unwrap_or(Ordering::Equal)
        }
    }

    impl<T> PartialOrd for Node<T>
    where
        T: Display + PartialEq + Eq + Hash + Clone,
    {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<T> PartialEq for Node<T>
    where
        T: Display + PartialEq + Eq + Hash + Clone,
    {
        fn eq(&self, other: &Self) -> bool {
            self.get_freq() == other.get_freq()
        }
    }

    impl<T> Eq for Node<T> where T: Display + PartialEq + Eq + Hash + Clone {}

    impl<T> Node<T>
    where
        T: Display + PartialEq + Eq + Hash + Clone,
    {
        fn get_freq(&self) -> f64 {
            match self {
                Self::Leaf { freq, .. } => *freq,
                Self::Internal { freq, .. } => *freq,
            }
        }

        fn make_tree(freq_list: Vec<(T, f64)>) -> Option<Self> {
            let mut node_list = freq_list
                .into_iter()
                .map(|(label, freq)| Node::Leaf { freq, label })
                .collect::<Vec<_>>();

            node_list.sort();
            node_list.reverse();

            while node_list.len() >= 2 {
                let n1 = node_list.pop().unwrap();
                let n2 = node_list.pop().unwrap();

                let new_node = Node::Internal {
                    freq: n1.get_freq() + n2.get_freq(),
                    left: Box::new(n1),
                    right: Box::new(n2),
                };

                // https://stackoverflow.com/questions/36249693/whats-the-most-efficient-way-to-insert-an-element-into-a-sorted-vector
                // https://stackoverflow.com/questions/57641712/is-there-an-efficient-function-in-rust-that-finds-the-index-of-the-first-occurre
                let pos = node_list
                    .binary_search_by(|p| p.cmp(&new_node).reverse())
                    .unwrap_or_else(|e| e);
                node_list.insert(pos, new_node);
            }

            node_list.pop()
        }

        fn get_huffman_tree(&self) -> (String, BiMap<T, String>) {
            let mut tree = String::from(".");
            let mut book = BiMap::new();

            self.huff_rec(&mut tree, "", "", &mut book);

            (tree, book)
        }

        fn huff_rec(
            &self,
            tree: &mut String,
            indent: &str,
            huff_code: &str,
            book: &mut BiMap<T, String>,
        ) {
            match self {
                Self::Leaf { freq, label } => {
                    book.insert(label.clone(), String::from(huff_code));
                    let s = format!("{} ({:.2}) {}\n", label, freq, huff_code);
                    tree.push_str(s.as_str());
                }
                Self::Internal { freq, left, right } => {
                    let s = format!("{} ({:.2})\n{}├── ", huff_code, freq, indent);
                    tree.push_str(s.as_str());
                    left.huff_rec(
                        tree,
                        format!("{}|   ", indent).as_str(),
                        format!("{}0", huff_code).as_str(),
                        book,
                    );
                    let s = format!("{}└── ", indent);
                    tree.push_str(s.as_str());
                    right.huff_rec(
                        tree,
                        format!("{}    ", indent).as_str(),
                        format!("{}1", huff_code).as_str(),
                        book,
                    );
                }
            }
        }
    }

    pub struct HuffmanCode<T>
    where
        T: Display + PartialEq + Eq + Hash + Clone,
    {
        freq_list: Vec<(T, f64)>,
        tree: String,
        book: BiMap<T, String>,
    }

    impl<T> HuffmanCode<T>
    where
        T: Display + PartialEq + Eq + Hash + Clone,
    {
        pub fn new(freq_list: Vec<(T, f64)>) -> Result<Self, String> {
            if freq_list.len() == 0 {
                return Err("freq_list's length is 0.".to_string());
            }
            let top = Node::make_tree(freq_list.clone()).unwrap();
            let (tree, book) = top.get_huffman_tree();

            Ok(HuffmanCode {
                freq_list,
                tree,
                book,
            })
        }

        pub fn get_avg_len(&self) -> f64 {
            self.freq_list
                .iter()
                .map(|(k, f)| (self.book.get_by_left(k).unwrap().len() as f64) * (*f))
                .sum::<f64>()
        }

        pub fn get_tree(&self) -> &str {
            self.tree.as_str()
        }
    }

    impl<T> Display for HuffmanCode<T>
    where
        T: Display + PartialEq + Eq + Hash + Clone,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = self
                .freq_list
                .iter()
                .map(|(k, f)| format!("{} ({}) => {}", k, f, self.book.get_by_left(k).unwrap()))
                .collect::<Vec<_>>()
                .join("\n");
            write!(f, "{}", s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::huffman::*;
    use std::fmt::Display;
    use std::hash::Hash;

    fn checker<T>(v: Vec<(T, f64)>, correct_len: f64, digit: u32)
    where
        T: Display + PartialEq + Eq + Hash + Clone,
    {
        let huf_code = HuffmanCode::new(v).unwrap();
        println!("{}", huf_code);
        let t = 10u32.pow(digit) as f64;
        let avg_len = (huf_code.get_avg_len() * t).round() / t;
        let correct_len = (correct_len * t).round() / t;
        assert_eq!(avg_len, correct_len);
    }

    #[test]
    fn test1() {
        let v = vec![
            ("B", 5.0 / 12.0),
            ("C", 3.0 / 12.0),
            ("A", 2.0 / 12.0),
            ("D", 1.0 / 12.0),
            ("E", 1.0 / 12.0),
        ];
        checker(v, 2.08, 2);
    }

    #[test]
    fn test2() {
        let v = vec![
            ("A", 0.36),
            ("B", 0.21),
            ("C", 0.17),
            ("D", 0.13),
            ("E", 0.09),
            ("F", 0.04),
        ];
        checker(v, 2.39, 2);
    }
}
