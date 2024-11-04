use crate::node::Node;
use crate::value::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct Trie {
    root: Option<Arc<Node>>,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: None }
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        let root = self.root.as_ref()?;
        let mut current = root;

        for c in key.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return None,
            }
        }

        current.value.clone()
    }

    pub fn put(&self, key: &str, value: impl Into<Value>) -> Self {
        let new_root = if let Some(root) = &self.root {
            Some(Self::put_helper(
                root.clone(),
                key.chars().collect(),
                0,
                value,
            ))
        } else {
            let mut new_node = Node::new();
            if key.is_empty() {
                new_node.value = Some(value.into());
            } else {
                new_node.children.insert(
                    key.chars().next().unwrap(),
                    Arc::new(Self::create_path(key.chars().skip(1).collect(), value)),
                );
            }
            Some(Arc::new(new_node))
        };

        Trie { root: new_root }
    }

    fn put_helper(
        node: Arc<Node>,
        key: Vec<char>,
        depth: usize,
        value: impl Into<Value>,
    ) -> Arc<Node> {
        let mut new_node = Node::new();

        // Copy existing value and children
        new_node.value = node.value.clone();
        new_node.children = node.children.clone();

        if depth == key.len() {
            new_node.value = Some(value.into());
        } else {
            let c = key[depth];
            let child = if let Some(child) = node.children.get(&c) {
                Self::put_helper(child.clone(), key, depth + 1, value)
            } else {
                Arc::new(Self::create_path(
                    key.into_iter().skip(depth + 1).collect(),
                    value,
                ))
            };
            new_node.children.insert(c, child);
        }

        Arc::new(new_node)
    }

    fn create_path(key: Vec<char>, value: impl Into<Value>) -> Node {
        let mut node = Node::new();
        if key.is_empty() {
            node.value = Some(value.into());
        } else {
            node.children.insert(
                key[0],
                Arc::new(Self::create_path(key[1..].to_vec(), value)),
            );
        }
        node
    }

    pub fn delete(&self, key: &str) -> Self {
        let new_root = match &self.root {
            None => None,
            Some(root) => Self::delete_helper(root.clone(), key.chars().collect(), 0),
        };

        Trie { root: new_root }
    }

    fn delete_helper(node: Arc<Node>, key: Vec<char>, depth: usize) -> Option<Arc<Node>> {
        if depth == key.len() {
            // If this node has children, keep it but remove the value
            if !node.children.is_empty() {
                let mut new_node = Node::new();
                new_node.children = node.children.clone();
                return Some(Arc::new(new_node));
            }
            return None;
        }

        let c = key[depth];
        let mut new_node = Node::new();
        new_node.value = node.value.clone();
        new_node.children = node.children.clone();

        if let Some(child) = node.children.get(&c) {
            if let Some(new_child) = Self::delete_helper(child.clone(), key, depth + 1) {
                new_node.children.insert(c, new_child);
            } else {
                new_node.children.remove(&c);
            }
        }

        if new_node.children.is_empty() && new_node.value.is_none() {
            None
        } else {
            Some(Arc::new(new_node))
        }
    }

    pub fn get_root(&self) -> Arc<Node> {
        self.root.clone().unwrap_or_else(|| Arc::new(Node::new()))
    }

    /// Create a new Trie from a root node
    pub fn from_node(root: Arc<Node>) -> Self {
        Trie { root: Some(root) }
    }

    /// Extract the root node from the Trie
    pub fn into_node(self) -> Arc<Node> {
        self.root.unwrap_or_else(|| Arc::new(Node::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_put_test() {
        let trie = Trie::new();
        let trie = trie.put("test-int", 233u32);
        let trie = trie.put("test-int2", 23333333u64);
        let trie = trie.put("test-string", "test".to_string());
        let _trie = trie.put("", "empty-key".to_string());
    }

    #[test]
    fn trie_structure_check() {
        let trie = Trie::new();
        let trie = trie.put("test", 233u32);
        assert_eq!(trie.get("test"), Some(Value::Int32(233)));

        // Ensure the trie structure matches expectations
        let root = trie.get_root();
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children.get(&'t').unwrap().children.len(), 1);
        assert_eq!(
            root.children
                .get(&'t')
                .unwrap()
                .children
                .get(&'e')
                .unwrap()
                .children
                .len(),
            1
        );
        assert_eq!(
            root.children
                .get(&'t')
                .unwrap()
                .children
                .get(&'e')
                .unwrap()
                .children
                .get(&'s')
                .unwrap()
                .children
                .len(),
            1
        );
        assert_eq!(
            root.children
                .get(&'t')
                .unwrap()
                .children
                .get(&'e')
                .unwrap()
                .children
                .get(&'s')
                .unwrap()
                .children
                .get(&'t')
                .unwrap()
                .children
                .len(),
            0
        );
    }

    #[test]
    fn basic_put_get_test() {
        let trie = Trie::new();

        // Put something
        let trie = trie.put("test", Value::Int32(233));
        assert_eq!(trie.get("test"), Some(Value::Int32(233)));

        // Put something else
        let trie = trie.put("test", Value::Int32(23333333));
        assert_eq!(trie.get("test"), Some(Value::Int32(23333333)));

        // Overwrite with another type
        let trie = trie.put("test", Value::String("23333333".to_string()));
        assert_eq!(
            trie.get("test"),
            Some(Value::String("23333333".to_string()))
        );

        // Get something that doesn't exist
        assert_eq!(trie.get("test-2333"), None);

        // Put something at root
        let trie = trie.put("", Value::String("empty-key".to_string()));
        assert_eq!(trie.get(""), Some(Value::String("empty-key".to_string())));
    }

    #[test]
    fn put_get_one_path() {
        let trie = Trie::new();

        let trie = trie.put("111", Value::Int32(111));
        let trie = trie.put("11", Value::Int32(11));
        let trie = trie.put("1111", Value::Int32(1111));
        let trie = trie.put("11", Value::Int32(22));

        assert_eq!(trie.get("11"), Some(Value::Int32(22)));
        assert_eq!(trie.get("111"), Some(Value::Int32(111)));
        assert_eq!(trie.get("1111"), Some(Value::Int32(1111)));
    }

    #[test]
    fn basic_delete_test1() {
        let trie = Trie::new();

        // Put something
        let trie = trie.put("test", Value::Int32(2333));
        assert_eq!(trie.get("test"), Some(Value::Int32(2333)));

        let trie = trie.put("te", Value::Int32(23));
        assert_eq!(trie.get("te"), Some(Value::Int32(23)));

        let trie = trie.put("tes", Value::Int32(233));
        assert_eq!(trie.get("tes"), Some(Value::Int32(233)));

        // Delete something
        let trie = trie.delete("test");
        let trie = trie.delete("tes");
        let trie = trie.delete("te");

        assert_eq!(trie.get("te"), None);
        assert_eq!(trie.get("tes"), None);
        assert_eq!(trie.get("test"), None);
    }

    #[test]
    fn basic_delete_test2() {
        let trie = Trie::new();

        // Put something
        let trie = trie.put("test", Value::Int32(2333));
        assert_eq!(trie.get("test"), Some(Value::Int32(2333)));

        let trie = trie.put("te", Value::Int32(23));
        assert_eq!(trie.get("te"), Some(Value::Int32(23)));

        let trie = trie.put("tes", Value::Int32(233));
        assert_eq!(trie.get("tes"), Some(Value::Int32(233)));

        let trie = trie.put("", Value::Int32(123));
        assert_eq!(trie.get(""), Some(Value::Int32(123)));

        // Delete something
        let trie = trie.delete("");
        let trie = trie.delete("te");
        let trie = trie.delete("tes");
        let trie = trie.delete("test");

        assert_eq!(trie.get(""), None);
        assert_eq!(trie.get("te"), None);
        assert_eq!(trie.get("tes"), None);
        assert_eq!(trie.get("test"), None);
    }

    #[test]
    fn delete_free_test() {
        let trie = Trie::new();

        let trie = trie.put("test", Value::Int32(2333));
        let trie = trie.put("te", Value::Int32(23));
        let trie = trie.put("tes", Value::Int32(233));

        let trie = trie.delete("tes");
        let trie = trie.delete("test");

        assert_eq!(
            trie.get_root()
                .children
                .get(&'t')
                .and_then(|child| child.children.get(&'e'))
                .map(|child| child.children.len()),
            Some(0)
        );

        let trie = trie.delete("te");
        assert_eq!(trie.get_root(), Arc::new(Node::new()));
    }

    #[test]
    fn copy_on_write_test1() {
        let empty_trie = Trie::new();

        // Put something
        let trie1 = empty_trie.put("test", Value::Int32(2333));
        let trie2 = trie1.put("te", Value::Int32(23));
        let trie3 = trie2.put("tes", Value::Int32(233));

        // Delete something
        let trie4 = trie3.delete("te");
        let trie5 = trie3.delete("tes");
        let trie6 = trie3.delete("test");

        // Check each snapshot
        assert_eq!(trie3.get("te"), Some(Value::Int32(23)));
        assert_eq!(trie3.get("tes"), Some(Value::Int32(233)));
        assert_eq!(trie3.get("test"), Some(Value::Int32(2333)));

        assert_eq!(trie4.get("te"), None);
        assert_eq!(trie4.get("tes"), Some(Value::Int32(233)));
        assert_eq!(trie4.get("test"), Some(Value::Int32(2333)));

        assert_eq!(trie5.get("te"), Some(Value::Int32(23)));
        assert_eq!(trie5.get("tes"), None);
        assert_eq!(trie5.get("test"), Some(Value::Int32(2333)));

        assert_eq!(trie6.get("te"), Some(Value::Int32(23)));
        assert_eq!(trie6.get("tes"), Some(Value::Int32(233)));
        assert_eq!(trie6.get("test"), None);
    }

    #[test]
    fn copy_on_write_test2() {
        let empty_trie = Trie::new();

        // Put something
        let trie1 = empty_trie.put("test", Value::Int32(2333));
        let trie2 = trie1.put("te", Value::Int32(23));
        let trie3 = trie2.put("tes", Value::Int32(233));

        // Override something
        let trie4 = trie3.put("te", Value::String("23".to_string()));
        let trie5 = trie3.put("tes", Value::String("233".to_string()));
        let trie6 = trie3.put("test", Value::String("2333".to_string()));

        // Check each snapshot
        assert_eq!(trie3.get("te"), Some(Value::Int32(23)));
        assert_eq!(trie3.get("tes"), Some(Value::Int32(233)));
        assert_eq!(trie3.get("test"), Some(Value::Int32(2333)));

        assert_eq!(trie4.get("te"), Some(Value::String("23".to_string())));
        assert_eq!(trie4.get("tes"), Some(Value::Int32(233)));
        assert_eq!(trie4.get("test"), Some(Value::Int32(2333)));

        assert_eq!(trie5.get("te"), Some(Value::Int32(23)));
        assert_eq!(trie5.get("tes"), Some(Value::String("233".to_string())));
        assert_eq!(trie5.get("test"), Some(Value::Int32(2333)));

        assert_eq!(trie6.get("te"), Some(Value::Int32(23)));
        assert_eq!(trie6.get("tes"), Some(Value::Int32(233)));
        assert_eq!(trie6.get("test"), Some(Value::String("2333".to_string())));
    }

    #[test]
    fn copy_on_write_test3() {
        let empty_trie = Trie::new();

        // Put something
        let trie1 = empty_trie.put("test", Value::Int32(2333));
        let trie2 = trie1.put("te", Value::Int32(23));
        let trie3 = trie2.put("", Value::Int32(233));

        // Override something
        let trie4 = trie3.put("te", Value::String("23".to_string()));
        let trie5 = trie3.put("", Value::String("233".to_string()));
        let trie6 = trie3.put("test", Value::String("2333".to_string()));

        // Check each snapshot
        assert_eq!(trie3.get("te"), Some(Value::Int32(23)));
        assert_eq!(trie3.get(""), Some(Value::Int32(233)));
        assert_eq!(trie3.get("test"), Some(Value::Int32(2333)));

        assert_eq!(trie4.get("te"), Some(Value::String("23".to_string())));
        assert_eq!(trie4.get(""), Some(Value::Int32(233)));
        assert_eq!(trie4.get("test"), Some(Value::Int32(2333)));

        assert_eq!(trie5.get("te"), Some(Value::Int32(23)));
        assert_eq!(trie5.get(""), Some(Value::String("233".to_string())));
        assert_eq!(trie5.get("test"), Some(Value::Int32(2333)));

        assert_eq!(trie6.get("te"), Some(Value::Int32(23)));
        assert_eq!(trie6.get(""), Some(Value::Int32(233)));
        assert_eq!(trie6.get("test"), Some(Value::String("2333".to_string())));
    }

    #[test]
    fn mixed_test() {
        let mut trie = Trie::new();
        let n = 23333;
        for i in 0..n {
            let key = format!("{:05}", i);
            let value = format!("value-{:#08}", i);
            trie = trie.put(&key, Value::String(value));
        }

        let trie_full = trie.clone();

        for i in (0..n).step_by(2) {
            let key = format!("{:05}", i);
            let value = format!("new-value-{:#08}", i);
            trie = trie.put(&key, Value::String(value));
        }

        let trie_override = trie.clone();

        for i in (0..n).step_by(3) {
            let key = format!("{:05}", i);
            trie = trie.delete(&key);
        }

        let trie_final = trie.clone();

        // Verify trie_full
        for i in 0..n {
            let key = format!("{:05}", i);
            let value = format!("value-{:#08}", i);
            assert_eq!(trie_full.get(&key), Some(Value::String(value)));
        }

        // Verify trie_override
        for i in 0..n {
            let key = format!("{:05}", i);
            if i % 2 == 0 {
                let value = format!("new-value-{:#08}", i);
                assert_eq!(trie_override.get(&key), Some(Value::String(value)));
            } else {
                let value = format!("value-{:#08}", i);
                assert_eq!(trie_override.get(&key), Some(Value::String(value)));
            }
        }

        // Verify final trie
        for i in 0..n {
            let key = format!("{:05}", i);
            if i % 3 == 0 {
                assert_eq!(trie_final.get(&key), None);
            } else if i % 2 == 0 {
                let value = format!("new-value-{:#08}", i);
                assert_eq!(trie_final.get(&key), Some(Value::String(value)));
            } else {
                let value = format!("value-{:#08}", i);
                assert_eq!(trie_final.get(&key), Some(Value::String(value)));
            }
        }
    }
}
