
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool
}

impl TrieNode {
    fn new() -> Self {
        TrieNode { 
            children: Default::default(), 
            is_end: false,
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie { 
            root: TrieNode::new() 
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.bytes() {
            let idx = (ch - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end = true;
    }

    pub fn starts_with(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for ch in prefix.bytes() {
            let idx = (ch - b'a') as usize;
            match &node.children[idx] {
                Some(child) => node = child,
                None => return vec![], // Prefix not found
            }
        }
        let mut results = Vec::new();
        let mut curr = prefix.to_string();
        Self::dfs(node, &mut curr, &mut results);
        results
    }

    fn dfs(node: &TrieNode, curr: &mut String, results: &mut Vec<String>) {
        if node.is_end {
            results.push(curr.clone());
        }
        for (i, child) in node.children.iter().enumerate() {
            if let Some(child_node) = child.as_ref() {
                curr.push((b'a' + i as u8) as char);
                Self::dfs(child_node, curr, results);
                curr.pop();
            }
        }
    }
}
