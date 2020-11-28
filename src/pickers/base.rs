use regex::Regex;
use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct TestCase {
    pub name: Option<String>,
    pub namespace: Vec<String>,
}

pub fn find_nearest(
    filename: &str,
    test: &str,
    namespace: &str,
    line_no: usize,
) -> Result<Option<TestCase>> {
    let test = Regex::new(test).unwrap();
    let namespace = Regex::new(namespace).unwrap();
    let indent = Regex::new(r"^(\s+).*").unwrap(); let mut file = File::open(filename).expect("opening file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("reading file");

    let lines: Vec<_> = text.lines().take(line_no).collect();

    let mut test_item = None;
    let mut indent_level = std::usize::MAX;
    for line in lines.iter().rev() {
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
