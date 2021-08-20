use super::base;

use anyhow::Result;

fn find_nearest(filename: &str, line_no: usize) -> Result<Option<base::TestCase>> {
    Ok(base::find_nearest(
        &filename,
        r"^\s*(async )?def (test_\w+)",
        Some(r"^\s*class (\w+) ?.*:"),
        line_no,
        false,
    )?)
}

pub fn get_command(filename: &str, line_no: Option<usize>, full: bool) -> Result<Option<String>> {
    if full {
        return Ok(Some(format!("pytest")));
    }
    match line_no {
        Some(ln) => {
            let mut test_case = find_nearest(&filename, ln)?;
            if let Some(t) = test_case.as_mut() {
                let mut namespace_path = t
                    .namespace
                    .iter()
                    .map(|x| x.values[1].to_string())
                    .collect::<Vec<String>>()
                    .join("::");
                if let Some(tn) = t.name.as_mut() {
                    if namespace_path.len() > 2 {
                        namespace_path = format!(
                            "{}::{}",
                            namespace_path,
                            tn.values[tn.values.len() - 1].to_string()
                        );
                    } else {
                        namespace_path = format!("{}", tn.values[tn.values.len() - 1].to_string());
                    }
                }
                // TODO: pick runner automatically
                let comm = format!("pytest {}::{}", filename, namespace_path);
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
    fn test_simple_thingy() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 16)
            .unwrap()
            .unwrap();

        assert_eq!(resp.clone().name.unwrap().no, 15);
        assert_eq!(resp.name.unwrap().values[1], "test_function".to_string());
        assert_eq!(resp.namespace.len(), 0);
    }

    #[test]
    fn test_simple_command() {
        let resp = get_command("./fixtures/python/pytest/test_stuff.py", Some(16), false)
            .unwrap()
            .unwrap();
        assert_eq!(
            resp,
            "pytest ./fixtures/python/pytest/test_stuff.py::test_function"
        );
    }

    #[test]
    fn test_simple_on_def() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 15)
            .unwrap()
            .unwrap();
        assert_eq!(resp.clone().name.unwrap().no, 15);
        assert_eq!(resp.name.unwrap().values[1], "test_function".to_string());
        assert_eq!(resp.namespace.len(), 0);
    }

    #[test]
    fn test_simple_on_async_def() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 20)
            .unwrap()
            .unwrap();
        assert_eq!(resp.clone().name.unwrap().no, 19);
        assert_eq!(
            resp.name.unwrap().values[2],
            "test_async_function".to_string()
        );
        assert_eq!(resp.namespace.len(), 0);
    }

    #[test]
    fn test_simple_async_def_command() {
        let resp = get_command("./fixtures/python/pytest/test_stuff.py", Some(20), false)
            .unwrap()
            .unwrap();
        assert_eq!(
            resp,
            "pytest ./fixtures/python/pytest/test_stuff.py::test_async_function"
        );
    }

    #[test]
    fn test_simple_on_empty() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 14)
            .unwrap()
            .unwrap();
        assert_eq!(resp.clone().name.unwrap().no, 11);
        assert_eq!(resp.name.unwrap().values[1], "test_method_obj".to_string());
        assert_eq!(resp.namespace.len(), 1);
    }

    #[test]
    fn test_simple_on_class() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 10)
            .unwrap()
            .unwrap();
        assert_eq!(resp.name.is_none(), true);
        assert_eq!(resp.namespace.len(), 1);
    }

    #[test]
    fn test_method_obj() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 12)
            .unwrap()
            .unwrap();
        assert_eq!(resp.clone().name.unwrap().no, 11);
        assert_eq!(resp.name.unwrap().values[1], "test_method_obj".to_string());
        assert_eq!(resp.namespace.len(), 1);
        assert_eq!(resp.namespace[0].no, 10);
        assert_eq!(resp.namespace[0].values[1], "TestClassObj".to_string());
    }

    #[test]
    fn test_method_out_nested() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 6)
            .unwrap()
            .unwrap();
        println!("{:?}", resp.clone());
        assert_eq!(resp.clone().name.unwrap().no, 6);
        assert_eq!(resp.name.unwrap().values[1], "test_method".to_string());
        assert_eq!(resp.namespace.len(), 1);
        assert_eq!(resp.namespace[0].no, 1);
        assert_eq!(resp.namespace[0].values[1], "TestClass".to_string());
    }

    #[test]
    fn test_nested() {
        let resp = find_nearest("./fixtures/python/pytest/test_stuff.py", 4)
            .unwrap()
            .unwrap();
        assert_eq!(resp.clone().name.unwrap().no, 3);
        assert_eq!(
            resp.name.unwrap().values[1],
            "test_nestedclass_method".to_string()
        );
        assert_eq!(resp.namespace.len(), 2);
        assert_eq!(resp.namespace[0].no, 1);
        assert_eq!(resp.namespace[0].values[1], "TestClass".to_string());
        assert_eq!(resp.namespace[1].no, 2);
        assert_eq!(resp.namespace[1].values[1], "TestNestedClass".to_string());
    }

    #[test]
    fn test_nested_command() {
        let resp = get_command("./fixtures/python/pytest/test_stuff.py", Some(4), false)
            .unwrap()
            .unwrap();
        assert_eq!(resp, "pytest ./fixtures/python/pytest/test_stuff.py::TestClass::TestNestedClass::test_nestedclass_method");
    }

    #[test]
    fn test_full_command() {
        let resp = get_command("./fixtures/python/pytest/test_stuff.py", None, true)
            .unwrap()
            .unwrap();
        assert_eq!(resp, "pytest");
    }
}
