mod nx_file;


#[cfg(test)]
mod tests {
    use crate::nx_file::{NXFile, NXNodeData};
    #[derive(Default)]
    struct MapleMap {
        id: i64,
        on_user_enter_script: String,
        can_fly: bool,
        bgm: &'static str,
    }

    #[test]
    fn load_map_0_test() {
        let res = NXFile::new("./nx_files/Map.nx");
        assert_eq!(res.is_ok(), true);
        let map_nx = res.unwrap();
        let map0 = map_nx.resolve("Map/Map0").unwrap();
        let map0_nodes = map_nx.get_node_children(map0).unwrap();

        let mut found_maps: Vec<MapleMap> = vec![];
        for map_img in map0_nodes.iter() {
            let map_id = map_img.name.replace(".img", "");
            // TODO fix this method.
            // The idea is that i can borrow the nodes without killing map_nx or the other nodes xd
            let map_nodes = map_nx.get_node_children(map_img).unwrap();
            let mut new_map = MapleMap::default();
            for node in map_nodes {
                match node.name.as_str() {
                    "info" => {
                        if node.has_children() {
                            for info_child in map_nx.get_node_children(node).unwrap(){
                                match node.name.as_str() {
                                    "bgm" => {
                                        if let NXNodeData::String(bgm_str) = &node.data {
                                            new_map.bgm = bgm_str;
                                        }
                                    }
                                    "fly" => {
                                        if let NXNodeData::Int64(can_fly) = &node.data {
                                            new_map.can_fly = can_fly > &0;
                                        }
                                    }
                                    "onUserEnter" => {
                                        if let NXNodeData::String(script_name) = &node.data {
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

}
