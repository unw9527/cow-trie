# COW-Trie
This repo implements a key-value store using a copy-on-write trie structure. 
The trie allows efficient retrieval, insertion, and deletion of values associated with variable-length string keys. 
It basically follows the description in [BusTub Fall 2023 Project 0](https://15445.courses.cs.cmu.edu/fall2023/project0/). 
Most test cases in this project are also from the BusTub project.

## Features
* Support basic key-value store operations such as `put`, `get`, and `delete`.
* **Copy-On-Write Mechanism**: Each operation (Get, Put, and Delete) creates a new version of the trie rather than modifying it in place. This allows access to previous states of the trie with minimal overhead.
* **Memory Efficiency**: The trie reuses existing nodes whenever possible to minimize memory usage, only creating new nodes when a change is made.

## Usage
### Creating a Trie
```Rust
use cow_trie::Trie;

let trie = Trie::new();
```

### Inserting a Value
```Rust
let trie = trie.put("key1", Value::Int32(123));
```

### Retrieving a Value
```Rust
if let Some(value) = trie.get("key1") {
    println!("Value: {:?}", value);
}
```

### Deleting a Value
```Rust
let trie = trie.delete("key1");
```

## TODO
- [] Add concurrent support.