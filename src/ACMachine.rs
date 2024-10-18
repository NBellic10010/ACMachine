use std::collections::HashMap;
use crate::trie_node::TrieNode;
use crate::trie_node::AcquireNode;

pub struct ACMachine<'a> {
    root: &'a TrieNode<'a>
}

pub trait BuildTrie {
    fn build_trie(&mut self, seq: &Vec<Vec<u8>>) -> Option<bool>;
    fn add_word(&mut self, word: &Vec<u8>) -> Option<bool>;
}

pub trait Match {
    fn match_main(&mut self, main_string: String) -> Vec<String>;
}

impl BuildTrie for ACMachine {
    fn build_trie(&mut self, seq: &Vec<Vec<u8>>) -> Option<bool> {
        let mut root_node = TrieNode{
            arr: vec![],
            cha: 0u8,
            fail_pointer: None
        };

        self.root = &root_node;
        let mut prev_node = &mut root_node;
        for word in seq {
            for ch in word {
                let pre_node_arr = &mut prev_node.arr;

                let newnode = prev_node.get_child_by_char(ch);
                if !newnode.is_none() {
                    prev_node = &mut newnode?;
                    continue;
                }

                let mut newarr = TrieNode {
                    arr: vec![],
                    cha: *ch,
                    fail_pointer: None
                };
                let newarrpt = &mut newarr;
                pre_node_arr.push(
                    newarr
                );
                prev_node = newarrpt;
            }
            prev_node = &mut root_node;
        }

        //build fail pointers
        for node in root_node.arr.iter_mut() {
            node.fail_pointer = Some(&mut root_node);
        }

        for ape_node in root_node.arr.iter_mut() {
            let parent_fp = ape_node.fail_pointer;
            self.build_fail_pointers(ape_node, parent_fp);
        }

        return Some(true);
    }

    fn add_word(&mut self, word: &Vec<u8>) -> Option<bool> {
        todo!()
    }
}

impl Match for ACMachine {
    fn match_main(&mut self, main_string: String) -> HashMap<String, i32> {
        if self.root == None {
            panic!("Root is empty!")
        }

        let start = self.root;

        for main_ch in main_string.chars() {
            let ch_u8 = main_ch as u8;

        }

        todo!()
    }
}