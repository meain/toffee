mod base;
mod python;
mod rust;
use anyhow::Result;

pub fn get_command(filename: &str, line_no: Option<usize>, full: bool) -> Result<Option<String>> {
    if filename.ends_with(".py") {
        python::get_command(filename, line_no, full)
    } else if filename.ends_with(".rs") {
        rust::get_command(filename, line_no, full)
    } else {
        eprintln!("Error: Unknown filetype for file {}", filename);
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_unknown() {
        let resp = get_command("./fixtures/python/pytest/test_stuff.unknown", Some(16), false).unwrap();
        assert_eq!(resp, None);
    }

    #[test]
    fn test_python_no_lineno() {
        let resp = get_command("./fixtures/python/pytest/test_stuff.py", None, false)
            .unwrap()
            .unwrap();
        assert_eq!(resp, "pytest ./fixtures/python/pytest/test_stuff.py");
    }

    #[test]
    fn test_python_simple_command() {
        let resp = get_command("./fixtures/python/pytest/test_stuff.py", Some(16), false)
            .unwrap()
            .unwrap();
        assert_eq!(
            resp,
            "pytest ./fixtures/python/pytest/test_stuff.py::test_function"
        );
    }

    #[test]
    fn test_python_nested_command() {
        let resp = get_command("./fixtures/python/pytest/test_stuff.py", Some(4), false)
            .unwrap()
            .unwrap();
        assert_eq!(resp, "pytest ./fixtures/python/pytest/test_stuff.py::TestClass::TestNestedClass::test_nestedclass_method");
    }
}
