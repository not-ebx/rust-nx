pub mod nx_file;
pub mod nx_node;


#[cfg(test)]
mod tests {
    use std::process::abort;
    use crate::nx_file::{NXFile};
    use crate::nx_node::{NXNodeData, NXNodeType};
    #[derive(Default)]
    struct MapleMap {
        id: i64,
        on_user_enter_script: String,
        can_fly: bool,
        bgm: String,
    }

    #[test]
    fn load_map_0_test() {
        let map_nx = NXFile::new("./nx_files/Map.nx").unwrap();
        let map0 = map_nx.resolve("Map/Map0").unwrap();
        let map0_nodes = map_nx.get_node_children(map0);

        let mut found_maps: Vec<MapleMap> = vec![];
        for map_img in map0_nodes.iter() {
            let map_id = map_img.name.replace(".img", "");
            // TODO fix this method.
            // The idea is that i can borrow the nodes without killing map_nx or the other nodes xd
            let map_nodes = map_nx.get_node_children(map_img);
            let mut new_map = MapleMap::default();
            for node in map_nodes {
                match node.name.as_str() {
                    "info" => {
                        if node.has_children() {
                            for info_child in map_nx.get_node_children(node) {
                                match info_child.name.as_str() {
                                    "bgm" => {
                                        if let NXNodeData::String(bgm_str) = &info_child.data {
                                            new_map.bgm = bgm_str.to_string();
                                        }
                                    }
                                    "fly" => {
                                        if let NXNodeData::Int64(can_fly) = &info_child.data {
                                            new_map.can_fly = can_fly > &0;
                                        }
                                    }
                                    "onUserEnter" => {
                                        if let NXNodeData::String(script_name) = &info_child.data {
                                            new_map.on_user_enter_script = script_name.to_string();
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
            new_map.id = map_id.parse().unwrap();
            found_maps.push(new_map);
        }
        assert_eq!(found_maps.len(), 25);
    }

    #[test]
    fn load_child_by_string() {
        let map_nx = NXFile::new("./nx_files/Map.nx").unwrap();
        let root_node = map_nx.get_root();
        match root_node {
            None => {
                abort(); // Test failed
            }
            Some(node) => {
                if let Some(test_node) = map_nx.get_node_child(node, "Map"){
                    print!("Found node 'Map'");
                    assert!(test_node.ntype == NXNodeType::Empty)
                } else {
                    abort();
                }
            }
        }
    }

}
