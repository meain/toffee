use super::base;

use anyhow::Result;
use std::path::Path;

fn find_nearest(filename: &str, line_no: usize) -> Result<Option<base::TestCase>> {
    Ok(base::find_nearest(
        &filename,
        r"^\s*func\s*\(?([^\)]*)\)?\s+(Test\w+|Example\w+)",
        None,
        line_no,
        false,
    )?)
}

// TODO: make verbose flag configurable
pub fn get_command(
    filename: &str,
    line_no: Option<usize>,
    full: bool,
    verbose: bool,
) -> Result<Option<String>> {
    let verbose_str = if verbose { " -v" } else { "" };
    if full {
        return Ok(Some(format!("go test{} ./...", verbose_str)));
    }
    let module_path = match Path::new(&filename).parent().ok_or_else(|| ".") {
        Ok(m) => m.to_string_lossy().to_string(),
        _ => ".".to_string(),
    };
    match line_no {
        Some(ln) => {
            let mut test_case = find_nearest(&filename, ln)?;
            if let Some(t) = test_case.as_mut() {
                let mut namespace_path = format!("");
                if let Some(tn) = t.name.as_mut() {
                    // FIXME(meain): This is super hacky
                    // https://github.com/stretchr/testify#suite-package
                    let mut suite = tn.values[tn.values.len() - 2].to_string();
                    if suite.len() != 0 {
                        suite = suite.split(" ").collect::<Vec<&str>>()[1]
                            .strip_prefix("*")
                            .unwrap()
                            .to_string();
                        suite = "Test".to_string() + suite.as_str() + "/"
                    }
                    namespace_path =
                        format!("{}{}", suite, tn.values[tn.values.len() - 1].to_string());
                }
                let comm = format!(
                    "go test{} -run '^{}$' {}",
                    verbose_str, namespace_path, module_path
                );
                return Ok(Some(comm));
            };
            Ok(None)
        }
        None => {
            let comm = format!("go test -v {}", module_path);
            return Ok(Some(comm));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_simple_thingy() {
        let resp = find_nearest("./fixtures/go/gotest/main_test.go", 21)
            .unwrap()
            .unwrap();

        assert_eq!(resp.clone().name.unwrap().no, 8);
        assert_eq!(
            resp.name.unwrap().values[2],
            "TestInputParseBasic".to_string()
        );
        assert_eq!(resp.namespace.len(), 0);
    }

    #[test]
    fn test_go_suite() {
        let resp = find_nearest("./fixtures/go/gotest/main_test.go", 32)
            .unwrap()
            .unwrap();

        assert_eq!(resp.clone().name.unwrap().no, 31);
        assert_eq!(
            resp.name.as_ref().unwrap().values[2],
            "TestNewThing".to_string()
        );
        assert_eq!(resp.name.unwrap().values[1], "suite *TestSuite".to_string());
        assert_eq!(resp.namespace.len(), 0);
    }

    #[test]
    fn test_go_file_command() {
        let resp = get_command("./fixtures/go/gotest/main_test.go", None, false, false)
            .unwrap()
            .unwrap();
        assert_eq!(resp, "go test -v ./fixtures/go/gotest");
    }

    #[test]
    fn test_go_simple_on_func() {
        let resp = find_nearest("./fixtures/go/gotest/main_test.go", 8)
            .unwrap()
            .unwrap();
        assert_eq!(resp.clone().name.unwrap().no, 8);
        assert_eq!(
            resp.name.unwrap().values[2],
            "TestInputParseBasic".to_string()
        );
        assert_eq!(resp.namespace.len(), 0);
    }

    #[test]
    fn test_go_simple_command_normal() {
        let resp = get_command("./fixtures/go/gotest/main_test.go", Some(21), false, false)
            .unwrap()
            .unwrap();
        assert_eq!(
            resp,
            "go test -run '^TestInputParseBasic$' ./fixtures/go/gotest"
        );
    }

    #[test]
    fn test_go_full_command_normal() {
        let resp = get_command("./fixtures/go/gotest/main_test.go", None, true, false)
            .unwrap()
            .unwrap();
        assert_eq!(resp, "go test ./...");
    }

    #[test]
    fn test_go_simple_command_verbose() {
        let resp = get_command("./fixtures/go/gotest/main_test.go", Some(21), false, true)
            .unwrap()
            .unwrap();
        assert_eq!(
            resp,
            "go test -v -run '^TestInputParseBasic$' ./fixtures/go/gotest"
        );
    }

    #[test]
    fn test_go_full_command_verbose() {
        let resp = get_command("./fixtures/go/gotest/main_test.go", None, true, true)
            .unwrap()
            .unwrap();
        assert_eq!(resp, "go test -v ./...");
    }

    #[test]
    fn test_go_suite_command_normal() {
        let resp = get_command("./fixtures/go/gotest/main_test.go", Some(32), false, false)
            .unwrap()
            .unwrap();
        assert_eq!(
            resp,
            "go test -run '^TestSuite/TestNewThing$' ./fixtures/go/gotest"
        );
    }
}
