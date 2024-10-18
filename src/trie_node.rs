#[derive(Debug, Clone, PartialEq)]
pub struct TrieNode<'a> {
    pub arr: Vec<TrieNode<'a>>,
    pub cha: u8,
    pub fail_pointer: Option<&'a TrieNode<'a>>
}

impl PartialEq for TrieNode<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cha == other.cha
    }

    fn ne(&self, other: &Self) -> bool {
        self.cha != other.cha
    }
}
pub trait AcquireNode {
    fn has(&self, ch: u8) -> bool;
    fn get_child_by_char(&self, ch: u8) -> Option<&TrieNode>;
}

impl AcquireNode for TrieNode<'_> {
    fn has(&self, ch: &u8) -> bool {
        self.arr.iter().any(|n| n.has(ch))
    }

    fn get_child_by_char(&self, ch: &u8) -> Option<&TrieNode> {
        for n in &self.arr {
            if n.has(&ch) {
                return Some(n);
            }
        }
        return None;
    }
}

impl TrieNode<'_> {
    pub fn build_fail_pointers(&self, cur_node: &mut TrieNode, parent_fp: Option<&TrieNode>) {
        //leaf node
        if cur_node.arr.is_empty() {
            return;
        }

        for node in cur_node.arr.iter_mut() {
            let same_child_node = parent_fp?.get_child_by_char(&node.cha);
            if !same_child_node.is_none() {
                node.fail_pointer = same_child_node;
            } else {
                node.fail_pointer = parent_fp;
            }
        }

        for node in cur_node.arr.iter_mut() {
            self.build_fail_pointers(node, self.fail_pointer?.unwrap())
        }
    }
}