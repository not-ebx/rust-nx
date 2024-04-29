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

### TODO
- [x] Fix and clean borrows, what a pain
- [ ] Make a Better Test
- [ ] Make a benchmark test
- [ ] Optimize and clean code
- [ ] Audio Nodes (With data)
- [ ] Bitmap nodes (With Data)