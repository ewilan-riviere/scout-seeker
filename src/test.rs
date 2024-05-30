#[cfg(test)]
mod tests {
    use crate::list_files_recursive;
    #[test]
    fn test_list_files() {
        let files = list_files_recursive("./src");
        println!("{:?}", files);
        assert_eq!(files.len(), 2);
        assert!(files.contains(&"./src/test.rs".to_string()));
        assert!(files.contains(&"./src/main.rs".to_string()));
    }
}
