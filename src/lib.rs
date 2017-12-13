// MIT License
//
// Copyright (c) 2017 Alexander Serebryakov
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.  
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! # Trie
//!
//! Trie is the library that implements the [trie](https://en.wikipedia.org/wiki/Trie).
//!
//! Trie is a generic data structure, written `Trie<T, U>` where `T` is node key type and `U` is a
//! value type.
//!
//! # Motivation
//!
//! Trie may be faster than other data structures in some cases.
//!
//! For example, `Trie` may be used as a replacement for `HashMap` in case of a dictionary where
//! the number of words in dictionary is significantly less than number of different words in the
//! input.
//!
//! # Usage
//!
//! ```rust
//! use gtrie::Trie;
//!
//! let mut t = Trie::new();
//!
//! t.add("this".chars(), 1);
//! t.add("trie".chars(), 2);
//! t.add("contains".chars(), 3);
//! t.add("a".chars(), 4);
//! t.add("number".chars(), 5);
//! t.add("of".chars(), 6);
//! t.add("words".chars(), 7);
//!
//! assert_eq!(t.has_key("number".chars()), true);
//! assert_eq!(t.has_key("not_existing_key".chars()), false);
//! assert_eq!(t.get_value("words".chars()), Some(7));
//! assert_eq!(t.get_value("none".chars()), None);
//! ```

mod trie_node;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Eq;
use std::clone::Clone;
use trie_node::TrieNode;


/// Prefix tree object
pub struct Trie<T: Eq + Clone, U: Clone> {
    /// Root of the prefix tree
    root: Rc<RefCell<TrieNode<T, U>>>,
}


impl<T: Eq + Clone, U: Clone> Trie<T, U> {
    /// Creates a new `Trie` object
    ///
    /// # Example
    ///
    /// ```rust
    /// use gtrie::Trie;
    ///
    /// let t = Trie::<char, String>::new();
    /// ```
    pub fn new() -> Trie<T, U> {
        Trie { root: Rc::new(RefCell::new(TrieNode::new(None, None))) }
    }


    /// Checks that trie is empty
    ///
    /// # Example
    ///
    /// ```rust
    /// use gtrie::Trie;
    ///
    /// let t = Trie::<char, f64>::new();
    /// assert_eq!(t.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.root.borrow().children.is_empty()
    }


    /// Adds a new key to the trie
    ///
    /// # Example
    ///
    /// ```rust
    /// use gtrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".chars();
    ///
    /// t.add(data, 42);
    /// assert_eq!(t.is_empty(), false);
    /// ```
    pub fn add<V: Iterator<Item = T>>(&mut self, key: V, value: U) {
        let mut node = self.root.clone();
        for c in key {
            let next_node = (*node).borrow_mut().add(&c);
            node = next_node;
        }

        (*node).borrow_mut().set_value(value);
    }


    /// Clears the trie
    ///
    /// # Example
    ///
    /// ```rust
    /// use gtrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".chars();
    ///
    /// t.add(data, String::from("test"));
    /// t.clear();
    /// assert_eq!(t.is_empty(), true);
    /// ```
    pub fn clear(&mut self) {
        (*self.root).borrow_mut().children.clear();
    }


    /// Looks for the key in trie
    ///
    /// # Example
    ///
    /// ```rust
    /// use gtrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".chars();
    /// let another_data = "notintest".chars();
    ///
    /// t.add(data.clone(), 42);
    ///
    /// assert_eq!(t.is_empty(), false);
    /// assert_eq!(t.has_key(data), true);
    /// assert_eq!(t.has_key(another_data), false);
    /// ```
    pub fn has_key<V: Iterator<Item = T>>(&self, key: V) -> bool {
        match self.find_node(key) {
            Some(node) => {
                if node.borrow().may_be_leaf() {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }


    /// Gets the value from the tree by key
    ///
    /// # Example
    ///
    /// ```rust
    /// use gtrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".chars();
    /// let another_data = "notintest".chars();
    ///
    /// t.add(data.clone(), 42);
    ///
    /// assert_eq!(t.get_value(data), Some(42));
    /// assert_eq!(t.get_value(another_data), None);
    /// ```
    pub fn get_value<V: Iterator<Item = T>>(&self, key: V) -> Option<U> {
        match self.find_node(key) {
            Some(node) => node.borrow().get_value(),
            None => None,
        }
    }


    /// Sets the value pointed by a key
    ///
    /// # Example
    ///
    /// ```rust
    /// use gtrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".chars();
    /// let another_data = "notintest".chars();
    ///
    /// t.add(data.clone(), 42);
    ///
    /// assert_eq!(t.get_value(data.clone()), Some(42));
    /// assert_eq!(t.set_value(data.clone(), 43), Ok(()));
    /// assert_eq!(t.get_value(data), Some(43));
    /// assert_eq!(t.set_value(another_data, 39), Err(()));
    /// ```
    pub fn set_value<V: Iterator<Item = T>>(&mut self, key: V, value: U) -> Result<(), ()> {
        match self.find_node(key) {
            Some(node) => {
                node.borrow_mut().set_value(value);
                Ok(())
            }
            None => Err(()),
        }
    }


    pub fn apply_on_value<V: Iterator<Item = T>, F: FnOnce(&mut U)>(&mut self, key: V, f: F) -> Result<(), ()> {
        match self.find_node(key) {
            Some(node) => {
                match node.borrow().get_value() {
                    Some(mut v) => {
                        f(&mut v);
                        node.borrow_mut().set_value(v);
                        Ok(())
                    },
                    None => Err(())
                }
            }
            None => Err(()),
        }
    }


    /// Finds the node in the trie by the key
    ///
    /// Internal API
    fn find_node<V: Iterator<Item = T>>(&self, key: V) -> Option<Rc<RefCell<TrieNode<T, U>>>> {
        let mut node = self.root.clone();

        for c in key {
            let mut _next_node = node.clone();

            match node.borrow().find(&c) {
                Some(child) => _next_node = child,
                None => return None,
            }

            node = _next_node;
        }

        Some(node.clone())
    }
}
