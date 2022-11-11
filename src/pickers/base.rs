use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: Option<WithLineNo>,
    pub namespace: Vec<WithLineNo>,
}
#[derive(Debug, Clone)]
pub struct WithLineNo {
    pub no: usize,
    pub values: Vec<String>,
}

fn get_exact_line(no: usize, line_no: usize, search_downwards: bool) -> usize {
    if search_downwards {
        line_no + no
    } else {
        line_no - no
    }
}

pub fn find_next(filename: &str, test: &str, line_no: usize) -> Result<Option<WithLineNo>> {
    let test = Regex::new(test).unwrap();
    let mut file = File::open(filename).expect("opening file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("reading file");
    let lines: Vec<_> = text.lines().skip(line_no).collect();

    for (i, line) in lines.iter().enumerate() {
        let t_caps = test.captures(line);
        if let Some(c) = t_caps {
            let mut values: Vec<String> = vec![];
            for v in c.iter() {
                let k = v.unwrap().as_str().to_string();
                values.push(k);
            }
            return Ok(Some(WithLineNo {
                no: get_exact_line(i, line_no, true),
                values,
            }));
        }
    }
    Ok(None)
}

pub fn find_nearest(
    filename: &str,
    test: &str,
    namespace: Option<&str>,
    line_no: usize,
    search_downwards: bool,
) -> Result<Option<TestCase>> {
    let test = Regex::new(test).unwrap();
    let namespace = match namespace {
        Some(ns) => Some(Regex::new(ns).unwrap()),
        None => None,
    };
    let indent = Regex::new(r"^(\s+).*").unwrap();
    let mut file = File::open(filename).expect("opening file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("reading file");

    let mut lines: Vec<_> = text.lines().take(line_no).collect();
    if !search_downwards {
        lines = lines.into_iter().rev().collect();
    }

    let mut test_item = None;
    let mut indent_level = std::usize::MAX;
    for (i, line) in lines.iter().enumerate() {
        if test_item.is_none() {
            let t_caps = test.captures(line);
            if let Some(c) = t_caps {
                let mut values: Vec<String> = vec![];
                for v in c.iter() {
                    if let Some(k) = v {
                        values.push(k.as_str().to_string())
                    }
                }
                test_item = Some(TestCase {
                    name: Some(WithLineNo {
                        no: get_exact_line(i, line_no, search_downwards),
                        values,
                    }),
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
        if let Some(ns_regex) = &namespace {
            let n_caps = ns_regex.captures(line);
            if let Some(n) = n_caps {
                let i_caps = indent.captures(line);
                let i_level = match i_caps {
                    Some(i) => i[1].len(),
                    None => 0,
                };
                if i_level < indent_level {
                    indent_level = i_level;
                    if let Some(t) = test_item.as_mut() {
                        let mut values: Vec<String> = vec![];
                        for v in n.iter() {
                            let k = v.unwrap().as_str().to_string();
                            values.push(k);
                        }
                        t.namespace.push(WithLineNo {
                            no: get_exact_line(i, line_no, search_downwards),
                            values,
                        });
                    } else {
                        let mut values: Vec<String> = vec![];
                        for v in n.iter() {
                            let k = v.unwrap().as_str().to_string();
                            values.push(k);
                        }
                        test_item = Some(TestCase {
                            name: None,
                            namespace: vec![WithLineNo {
                                no: get_exact_line(i, line_no, search_downwards),
                                values,
                            }],
                        })
                    }
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

pub fn get_project_root<'a>(filename: &'a str, marker: &'a str) -> &'a str {
    let mut root = Path::new(filename.clone());
    // in case we are already at root
    if root.join(marker).exists() {
        return root.to_str().unwrap();
    }
    loop {
        match root.parent() {
            Some(p) => {
                if p.join(marker).exists() {
                    return p.to_str().unwrap();
                }
                root = p
            }
            None => return root.to_str().unwrap(),
        }
    }
}
