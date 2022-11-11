use super::base;

use anyhow::anyhow;
use anyhow::Result;

fn find_nearest_test_markers(filename: &str, line_no: usize) -> Result<Option<base::TestCase>> {
    Ok(base::find_nearest(
        &filename,
        r"^\s*#\[test\]",
        Some(r"^\s*#\[cfg\(test\)\]"),
        line_no,
        false,
    )?)
}

fn find_nearest_namespace(filename: &str, line_no: usize) -> Result<Option<base::WithLineNo>> {
    Ok(base::find_next(
        &filename,
        r"^\s*mod ([a-z_0-9]+)",
        line_no,
    )?)
}

fn find_nearest_test_function(filename: &str, line_no: usize) -> Result<Option<base::WithLineNo>> {
    Ok(base::find_next(
        &filename,
        r"^\s*fn test_([a-z_0-9]*)",
        line_no,
    )?)
}

pub fn get_command(
    filename: &str,
    line_no: Option<usize>,
    full: bool,
    verbose: bool,
) -> Result<Option<String>> {
    let verbose_str = if verbose { " -v" } else { "" };
    if full {
        return Ok(Some(format!("cargo test{}", verbose_str)));
    }

    // ran into an issue where we had path like
    // /home/meain/dev/src/project/src/blah/blah which caused issues
    // in the 'src/' split a few lines below
    let root = base::get_project_root(filename, "Cargo.toml");
    let relative_filename = &if filename.starts_with(root) {
        root.to_string().push_str("/");
        filename.replacen(root, "", 1)
    } else {
        filename.to_string()
    };
    match line_no {
        Some(ln) => {
            let test_markers = find_nearest_test_markers(&filename, ln)?;

            let file_namespace = relative_filename.split("src/").collect::<Vec<&str>>()[1];
            let file_namespace = file_namespace.replace("/", "::").replace(".rs", "");

            let mut comm = format!("cargo test{} {}", verbose_str, file_namespace);

            if let Some(tm) = test_markers {
                let ns = find_nearest_namespace(filename, tm.namespace[0].no)?;
                if let Some(n) = ns {
                    comm = format!("{}::{}", comm, n.values[1]);
                } else {
                    return Err(anyhow!("Could not find mod."));
                }

                if let Some(test_name) = tm.name {
                    let ns = find_nearest_test_function(filename, test_name.no)?;
                    if let Some(n) = ns {
                        comm = format!("{}::test_{}", comm, n.values[1]);
                    } else {
                        return Err(anyhow!("Could not find test function."));
                    }
                }
            }

            return Ok(Some(comm));
        }
        None => {
            let file_namespace = relative_filename.split("src/").collect::<Vec<&str>>()[1];
            let file_namespace = file_namespace.replace("/", "::").replace(".rs", "");
            let comm = format!("cargo test {}", file_namespace);
            return Ok(Some(comm));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_find() {
        let resp = find_nearest_test_markers("./fixtures/rust/cargo/src/pickers/tester.rs", 16)
            .unwrap()
            .unwrap();
        assert_eq!(resp.clone().name.unwrap().no, 5);
        assert_eq!(resp.name.unwrap().values.len(), 1);
        assert_eq!(resp.namespace.len(), 1);
        assert_eq!(resp.namespace[0].no, 1);
        assert_eq!(resp.namespace[0].values.len(), 1);
    }

    #[test]
    fn test_simple_command() {
        let resp = get_command(
            "./fixtures/rust/cargo/src/pickers/tester.rs",
            Some(16),
            false,
            false,
        )
        .unwrap()
        .unwrap();
        assert_eq!(resp, "cargo test pickers::tester::tests::test_simple");
    }

    #[test]
    fn test_mod_command_normal() {
        let resp = get_command(
            "./fixtures/rust/cargo/src/pickers/tester.rs",
            Some(3),
            false,
            false,
        )
        .unwrap()
        .unwrap();
        assert_eq!(resp, "cargo test pickers::tester::tests");
    }

    #[test]
    fn test_full_command_normal() {
        let resp = get_command(
            "./fixtures/rust/cargo/src/pickers/tester.rs",
            Some(3),
            true,
            false,
        )
        .unwrap()
        .unwrap();
        assert_eq!(resp, "cargo test");
    }

    #[test]
    fn test_mod_command_verbose() {
        let resp = get_command(
            "./fixtures/rust/cargo/src/pickers/tester.rs",
            Some(3),
            false,
            true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(resp, "cargo test -v pickers::tester::tests");
    }

    #[test]
    fn test_full_command_verbose() {
        let resp = get_command(
            "./fixtures/rust/cargo/src/pickers/tester.rs",
            Some(3),
            true,
            true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(resp, "cargo test -v");
    }
}
