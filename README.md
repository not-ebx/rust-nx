# rustNX
A rust implementation of a [PKNX4](https://nxformat.github.io/) format reader.

### Features
- [x] Tree-like iteration on nodes
- [x] String Nodes
- [x] Long and Double
- [x] Vectors node
- [x] Audio Nodes (Without data, just the info)
- [x] Bitmap nodes (Without data, just the info)

For the meantime, i have no intention of implementing the Audio and Bitmap nodes, since there's no use for them when developing a server. Maybe ill add them.... who knows.

### Installation
To use on your project, you must include it as such, inside your Cargo.toml
```toml
[dependencies]
rust_nx = { git = "https://github.com/not-ebx/rust-nx", tag = "v0.1.4" }
```
The tag can change and I may forget to update the readme with the latest version. You can always check the latest tag by clicking on the releases.

### Usage
Using this is pretty simple. There's an example on src/lib.rs.
But essentially it's:
```rust
// First, open your desired NXFile
let map_nx = NXFile::new("./nx_files/Map.nx").unwrap();
// Then open a node inside of it
let map0 = map_nx.resolve("Map/Map0").unwrap();
```
Once you resolve to a node, you can iterate like so:
```rust
for nodes in map_nx.get_node_children(map0).iter() {
    //.. Do whatever you want xd
}
```

Or you can get a single child by doing
```rust
map_nx.get_node_child(map0, "child_name");
```
This returns an Option<&NXNode>

### TODO
- [x] Fix and clean borrows, what a pain
- [ ] Make a Better Test
- [ ] Make a benchmark test
- [ ] Optimize and clean code
- [ ] Audio Nodes (With data)
- [ ] Bitmap nodes (With Data)

### Credits
- PKNX4 Format specification, found at https://nxformat.github.io/
- aatxe's pkgnx's implementation, helped me understand some stuff i didnt quite get from the format definition. Found https://github.com/aatxe/pkgnx