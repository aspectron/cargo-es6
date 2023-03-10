use crate::prelude::*;
// pub use globset::{Glob,GlobMatcher};

#[derive(Debug, Default)]
pub struct Filter {
    matchers: Vec<GlobMatcher>,
    negators: Vec<GlobMatcher>,
}

// impl Default for Filter {
//     fn default() -> Self {
//         Filter {
//             matchers: Vec::new(),
//             negators: Vec::new(),
//         }
//     }
// }

impl Filter {
    pub fn new(globs: &[&str]) -> Filter {
        let mut matchers = Vec::new();
        let mut negators = Vec::new();
        let negator_prefix = Regex::new(r"^!/?").unwrap();

        for glob in globs {
            if negator_prefix.is_match(glob) {
                //glob.starts_with("!") {
                let glob = negator_prefix.replace(glob, ""); //&glob[1..];
                negators.push(
                    Glob::new(&glob)
                        .unwrap_or_else(|_| panic!("Error compiling glob: {glob}"))
                        .compile_matcher(),
                );
            } else {
                matchers.push(
                    Glob::new(glob)
                        .unwrap_or_else(|_| panic!("Error compiling glob: {glob}"))
                        .compile_matcher(),
                );
            };
        }
        Filter { matchers, negators }
    }

    pub fn is_match(&self, text: &str) -> bool {
        !self.negators.iter().any(|filter| filter.is_match(text))
            && self.matchers.iter().any(|filter| filter.is_match(text))
    }
}
