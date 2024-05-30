#[cfg(test)]
mod tests {
    use crate::list_files_recursive;
    #[test]
    fn test_list_files() {
        let files = list_files_recursive("./src");
        println!("{:?}", files);
        assert_eq!(files.len(), 2);
        assert_eq!(files, vec!["./src/test.rs", "./src/main.rs"]);
    }
}
