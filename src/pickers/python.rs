use super::base;

use anyhow::Result;
// use std::fs;

fn find_nearest(filename: &str, line_no: usize) -> Result<Option<base::TestCase>> {
    Ok(base::find_nearest(
        &filename,
        r"^ *def (test_\w+)",
        r"^\s*class (\w+) ?.*:",
        line_no,
    )?)
}

pub fn get_command(filename: &str, line_no: Option<usize>) -> Result<Option<String>> {
    match line_no {
        Some(ln) => {
            let mut test_case = find_nearest(&filename, ln)?;
            if let Some(t) = test_case.as_mut() {
                // TODO: pick runner automatically
                if let Some(tn) = t.name.as_mut() {
                    t.namespace.push(tn.to_string());
                }
                let namespace_path = t.namespace.join("::");
                let comm = format!(
                    "pytest {}::{}",
                    // fs::canonicalize(filename)?.to_string_lossy(),
                    filename,
                    namespace_path
                );
                return Ok(Some(comm));
            };
            Ok(None)
        }
        None => {
            let comm = format!("pytest {}", filename,);
            return Ok(Some(comm));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_simple() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 16)
            .unwrap()
            .unwrap();
        assert_eq!(resp.name, Some("test_function".to_string()));
        assert_eq!(resp.namespace.len(), 0);
    }

    #[test]
    fn test_python_simple_command() {
        let resp = get_command("./fixtures/python/pytest/test_stuff.py", 16)
            .unwrap()
            .unwrap();
        assert_eq!(
            resp,
            "pytest ./fixtures/python/pytest/test_stuff.py::test_function"
        );
    }

    #[test]
    fn test_python_simple_on_def() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 15)
            .unwrap()
            .unwrap();
        assert_eq!(resp.name, Some("test_function".to_string()));
        assert_eq!(resp.namespace.len(), 0);
    }

    #[test]
    fn test_python_simple_on_empty() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 14)
            .unwrap()
            .unwrap();
        assert_eq!(resp.name, Some("test_method_obj".to_string()));
        assert_eq!(resp.namespace.len(), 1);
    }

    #[test]
    fn test_python_simple_on_class() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 10)
            .unwrap()
            .unwrap();
        assert_eq!(resp.name, None);
        assert_eq!(resp.namespace.len(), 1);
    }

    #[test]
    fn test_python_method_obj() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 12)
            .unwrap()
            .unwrap();
        assert_eq!(resp.name, Some("test_method_obj".to_string()));
        assert_eq!(resp.namespace.len(), 1);
        let actual_namespace: Vec<String> = vec!["TestClassObj".to_string()];
        assert_eq!(resp.namespace, actual_namespace);
    }

    #[test]
    fn test_python_method_out_nested() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 6)
            .unwrap()
            .unwrap();
        assert_eq!(resp.name, Some("test_method".to_string()));
        assert_eq!(resp.namespace.len(), 1);
        let actual_namespace: Vec<String> = vec!["TestClass".to_string()];
        assert_eq!(resp.namespace, actual_namespace);
    }

    #[test]
    fn test_python_nested() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 4)
            .unwrap()
            .unwrap();
        assert_eq!(resp.name, Some("test_nestedclass_method".to_string()));
        assert_eq!(resp.namespace.len(), 2);
        let actual_namespace: Vec<String> =
            vec!["TestClass".to_string(), "TestNestedClass".to_string()];
        assert_eq!(resp.namespace, actual_namespace);
    }

    #[test]
    fn test_python_nested_command() {
        let resp = get_command("./fixtures/python/pytest/test_stuff.py", 4)
            .unwrap()
            .unwrap();
        assert_eq!(resp, "pytest ./fixtures/python/pytest/test_stuff.py::TestClass::TestNestedClass::test_nestedclass_method");
    }
}
