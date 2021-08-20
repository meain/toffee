use super::base;

use anyhow::Result;

fn find_nearest(filename: &str, line_no: usize) -> Result<Option<base::TestCase>> {
    Ok(base::find_nearest(
        &filename,
        r"^\s*func (Test\w+)",
        None,
        line_no,
        false,
    )?)
}

pub fn get_command(filename: &str, line_no: Option<usize>, full: bool) -> Result<Option<String>> {
    if full {
        return Ok(Some(format!("go test")));
    }
    match line_no {
        Some(ln) => {
            let mut test_case = find_nearest(&filename, ln)?;
            if let Some(t) = test_case.as_mut() {
                let mut namespace_path = format!("");
                if let Some(tn) = t.name.as_mut() {
                    namespace_path = format!("{}", tn.values[tn.values.len() - 1].to_string());
                }
                let comm = format!("go test -run {}", namespace_path);
                return Ok(Some(comm));
            };
            Ok(None)
        }
        None => {
            let comm = format!("go test {}", filename,);
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
            resp.name.unwrap().values[1],
            "TestInputParseBasic".to_string()
        );
        assert_eq!(resp.namespace.len(), 0);
    }

    #[test]
    fn test_go_simple_command() {
        let resp = get_command("./fixtures/go/gotest/main_test.go", Some(21), false)
            .unwrap()
            .unwrap();
        assert_eq!(resp, "go test -run TestInputParseBasic");
    }

    #[test]
    fn test_go_simple_on_func() {
        let resp = find_nearest("./fixtures/go/gotest/main_test.go", 8)
            .unwrap()
            .unwrap();
        assert_eq!(resp.clone().name.unwrap().no, 8);
        assert_eq!(resp.name.unwrap().values[1], "TestInputParseBasic".to_string());
        assert_eq!(resp.namespace.len(), 0);
    }

    #[test]
    fn test_go_full_command() {
        let resp = get_command("./fixtures/go/gotest/main_test.go", None, true)
            .unwrap()
            .unwrap();
        assert_eq!(resp, "go test");
    }
}
