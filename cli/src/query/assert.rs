use super::super::error::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use tree_sitter::Point;

// TODO: It would be cooler to do this with a comments query rather than with a regex
// directly.
lazy_static! {
    static ref METADATA_REGEX: Regex = Regex::new(r#"(\w+): ([^\s,]+), (\d+), (\d+)"#).unwrap();
}

pub struct CaptureInfo {
    pub name: String,
}

#[derive(Debug, Eq, PartialEq)]
struct Assertion {
    position: Point,
    capture_class: String,
    capture_type: String,
}

impl From<regex::Captures<'_>> for Assertion {
    fn from(re: regex::Captures) -> Assertion {
        Assertion {
            capture_class: re.get(1).unwrap().as_str().to_string(),
            capture_type: re.get(2).unwrap().as_str().to_string(),
            position: Point {
                row: re
                    .get(3)
                    .iter()
                    .flat_map(|m| m.as_str().parse::<usize>())
                    .next()
                    .unwrap(),
                column: re
                    .get(4)
                    .iter()
                    .flat_map(|m| m.as_str().parse::<usize>())
                    .next()
                    .unwrap(),
            },
        }
    }
}

pub fn assert_expected_captures(_captures: Vec<CaptureInfo>, path: String) -> Result<()> {
    let contents = fs::read_to_string(path)?;

    let assertions: Vec<Assertion> = METADATA_REGEX
        .captures_iter(&contents)
        .map(|c| Assertion::from(c))
        .collect();

    for a in assertions {
        println!("a: {:?}", a);
    }

    Ok(())
}
