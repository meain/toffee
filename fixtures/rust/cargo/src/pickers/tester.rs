#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let resp = find_nearest_test_markers("./fixtures/rust/cargo/src/pickers/tester.rs", 16)
            .unwrap()
            .unwrap();
        assert_eq!(
            resp.name.unwrap(),
            WithLineNo {
                no: 15,
                value: "test_function".to_string()
            }
        );
        assert_eq!(resp.namespace.len(), 0);
    }
}
