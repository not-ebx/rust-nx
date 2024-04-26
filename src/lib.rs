mod nx_file;


#[cfg(test)]
mod tests {
    use crate::nx_file::NXFile;
    use super::*;

    #[test]
    fn it_works() {
        let res = NXFile::new("/Users/knux/Documents/Programming/self/OSS-Projects/rustNX/String.nx");
        assert_eq!(res.is_ok(), true);

        let mut nx_file = res.unwrap();
        let parser = nx_file.parse();

        assert_eq!(parser.is_ok(), true);
    }
}
