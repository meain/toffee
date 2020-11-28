use regex::Regex;
use std::env;
use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct TestCase {
    name: Option<String>,
    namespace: Vec<String>,
}

pub fn find_nearest_test(
    filename: &str,
    test: &str,
    namespace: &str,
    line_no: usize,
) -> Result<Option<TestCase>> {
    let test = Regex::new(test).unwrap();
    let namespace = Regex::new(namespace).unwrap();
    let indent = Regex::new(r"^(\s+).*").unwrap();

    let mut file = File::open(filename).expect("opening file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("reading file");

    let lines: Vec<_> = text.lines().take(line_no).collect();
    println!("lines: {:?}", lines.len());

    let mut test_item = None;
    let mut indent_level = std::usize::MAX;
    for line in lines.iter().rev() {
        println!("Checking: {} {}", indent_level, line);
        if test_item.is_none() {
            let t_caps = test.captures(line);
            if let Some(c) = t_caps {
                test_item = Some(TestCase {
                    name: Some(c[1].to_string()),
                    namespace: Vec::new(),
                });

                // update indent level
                let i_caps = indent.captures(line);
                let i_level = match i_caps {
                    Some(i) => i[1].len(),
                    None => 0,
                };
                if i_level < indent_level {
                    indent_level = i_level;
                }
                continue;
            }
        }

        // checking namespace
        let n_caps = namespace.captures(line);
        if let Some(n) = n_caps {
            let i_caps = indent.captures(line);
            let i_level = match i_caps {
                Some(i) => i[1].len(),
                None => 0,
            };
            if i_level < indent_level {
                indent_level = i_level;
                if let Some(t) = test_item.as_mut() {
                    t.namespace.push(n[1].to_string());
                } else {
                    test_item = Some(TestCase {
                        name: None,
                        namespace: vec![n[1].to_string()],
                    })
                }
            }
        }

        if indent_level == 0 {
            break;
        }
    }

    if let Some(t) = test_item.as_mut() {
        t.namespace = t.namespace.clone().into_iter().rev().collect();
    }
    Ok(test_item)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();

    let te = find_nearest_test(&filename, r"^ *def (test_\w+)", r"^\s*class (\w+) ?.*:", 16);
    if let Ok(t) = te {
        println!("t: {:?}", t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_simple() {
        let resp = find_nearest_test(
            "./fixtures/python/pytest/test_stuff.py",
            r"^ *def (test_\w+)",
            r"^\s*class (\w+) ?.*:",
            16,
        )
        .unwrap()
        .unwrap();
        assert_eq!(resp.name, Some("test_function".to_string()));
        assert_eq!(resp.namespace.len(), 0);
    }

    #[test]
    fn test_python_simple_on_def() {
        let resp = find_nearest_test(
            "./fixtures/python/pytest/test_stuff.py",
            r"^ *def (test_\w+)",
            r"^\s*class (\w+) ?.*:",
            15,
        )
        .unwrap()
        .unwrap();
        assert_eq!(resp.name, Some("test_function".to_string()));
        assert_eq!(resp.namespace.len(), 0);
    }

    #[test]
    fn test_python_simple_on_empty() {
        let resp = find_nearest_test(
            "./fixtures/python/pytest/test_stuff.py",
            r"^ *def (test_\w+)",
            r"^\s*class (\w+) ?.*:",
            14,
        )
        .unwrap()
        .unwrap();
        assert_eq!(resp.name, Some("test_method_obj".to_string()));
        assert_eq!(resp.namespace.len(), 1);
    }

    #[test]
    fn test_python_simple_on_class() {
        let resp = find_nearest_test(
            "./fixtures/python/pytest/test_stuff.py",
            r"^ *def (test_\w+)",
            r"^\s*class (\w+) ?.*:",
            10,
        )
        .unwrap()
        .unwrap();
        assert_eq!(resp.name, None);
        assert_eq!(resp.namespace.len(), 1);
    }

    #[test]
    fn test_python_method_obj() {
        let resp = find_nearest_test(
            "./fixtures/python/pytest/test_stuff.py",
            r"^ *def (test_\w+)",
            r"^\s*class (\w+) ?.*:",
            12,
        )
        .unwrap()
        .unwrap();
        assert_eq!(resp.name, Some("test_method_obj".to_string()));
        assert_eq!(resp.namespace.len(), 1);
        let actual_namespace: Vec<String> = vec!["TestClassObj".to_string()];
        assert_eq!(resp.namespace, actual_namespace);
    }

    #[test]
    fn test_python_method_out_nested() {
        let resp = find_nearest_test(
            "./fixtures/python/pytest/test_stuff.py",
            r"^ *def (test_\w+)",
            r"^\s*class (\w+) ?.*:",
            6,
        )
        .unwrap()
        .unwrap();
        assert_eq!(resp.name, Some("test_method".to_string()));
        assert_eq!(resp.namespace.len(), 1);
        let actual_namespace: Vec<String> = vec!["TestClass".to_string()];
        assert_eq!(resp.namespace, actual_namespace);
    }

    #[test]
    fn test_python_nested() {
        let resp = find_nearest_test(
            "./fixtures/python/pytest/test_stuff.py",
            r"^ *def (test_\w+)",
            r"^\s*class (\w+) ?.*:",
            4,
        )
        .unwrap()
        .unwrap();
        assert_eq!(resp.name, Some("test_nestedclass_method".to_string()));
        assert_eq!(resp.namespace.len(), 2);
        let actual_namespace: Vec<String> =
            vec!["TestClass".to_string(), "TestNestedClass".to_string()];
        assert_eq!(resp.namespace, actual_namespace);
    }
}
