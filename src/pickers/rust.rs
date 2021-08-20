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

pub fn get_command(filename: &str, line_no: Option<usize>, full: bool) -> Result<Option<String>> {
    if full {
        return Ok(Some(format!("cargo test")));
    }
    match line_no {
        Some(ln) => {
            let test_markers = find_nearest_test_markers(&filename, ln)?;

            let file_namespace = filename.split("src/").collect::<Vec<&str>>()[1];
            let file_namespace = file_namespace.replace("/", "::").replace(".rs", "");

            let mut comm = format!("cargo test {}", file_namespace);

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
            let file_namespace = filename.split("src/").collect::<Vec<&str>>()[1];
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
        )
        .unwrap()
        .unwrap();
        assert_eq!(resp, "cargo test pickers::tester::tests::test_simple");
    }

    #[test]
    fn test_mod_command() {
        let resp = get_command(
            "./fixtures/rust/cargo/src/pickers/tester.rs",
            Some(3),
            false,
        )
        .unwrap()
        .unwrap();
        assert_eq!(resp, "cargo test pickers::tester::tests");
    }

    #[test]
    fn test_full_command() {
        let resp = get_command("./fixtures/rust/cargo/src/pickers/tester.rs", Some(3), true)
            .unwrap()
            .unwrap();
        assert_eq!(resp, "cargo test");
    }
}
