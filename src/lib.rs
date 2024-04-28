mod nx_file;


#[cfg(test)]
mod tests {
    use crate::nx_file::NXFile;

    #[test]
    fn it_works() {
        let res = NXFile::new("/Users/knux/Documents/Programming/self/OSS-Projects/rustNX/test/String.nx");
        assert_eq!(res.is_ok(), true);
        let string_nx = res.unwrap();
        let cash_img = string_nx.resolve("Cash.img").unwrap();
        let cash_items = string_nx.get_node_children(cash_img).unwrap();
        for item in cash_items.iter() {
            let item_nodes = string_nx.get_node_children(item).unwrap();
            for node in item_nodes.iter() {
                match node.name.as_str() {
                    "name" => {
                        print!("Name: {:?}\n", node.data)
                    },
                    "desc" => {
                        print!("Desc: {:?}\n", node.data)
                    },
                    _ => {}
                }
            }
        }
    }
}
