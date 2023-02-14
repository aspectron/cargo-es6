use crate::prelude::*;

pub fn gather_references(text: &str, referrer: u64) -> Result<(Option<Vec<Reference>>, String)> {
    let mut references = Vec::new();
    // handle `import xxx from "xxx"`
    let import_re = Regex::new(r###"import[^;]*from\s*["'][^"']+["'];"###).unwrap();
    let import_reference_re = Regex::new(r###"import\s*(.+)\s*from\s*["'](.+)["']"###).unwrap();
    let import_matches = import_re
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<_>>();
    for import in import_matches.iter() {
        let import = import.replace('\n', " ");
        let captures = import_reference_re
            .captures(&import)
            .unwrap_or_else(|| panic!("unable to capture `{import}`"));
        let what = captures[1].to_string();
        let location = captures[2].to_string();
        let import = Reference::new(
            ReferenceKind::Module,
            referrer, //.as_ref(),
            Some(&what),
            &location,
        );
        references.push(import);
    }
    let text = import_re.replace_all(text, "");

    // handle `import "..."
    // let import_re = Regex::new(r###"^\s*import\s*["'][^"']+["'];"###).unwrap();
    let import_re = Regex::new(r###"import\s*["'][^"']+["'];"###).unwrap();
    let import_reference_re = Regex::new(r###"import\s*["'](.+)["']"###).unwrap();
    let import_matches = import_re
        .find_iter(&text)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<_>>();
    for import in import_matches.iter() {
        // println!("| import len: {}", import.len());
        // println!("| import match: {} {}", import,import.len());
        let import = import.replace('\n', " ");
        let captures = import_reference_re
            .captures(&import)
            .unwrap_or_else(|| panic!("unable to capture `{import}`"));
        // let what = "*".to_string();
        let location = captures[1].to_string();
        // println!("| import location: {}", location);
        let import = Reference::new(ReferenceKind::Module, referrer, None, &location);
        references.push(import);
    }
    let text = import_re.replace_all(&text, "");

    // let mut exports = Vec::new();
    let export_re = Regex::new(r###"export[^;]*from\s*["']+[^"']+["']+;"###).unwrap();
    let export_reference_re = Regex::new(r###"export\s*(.+)\s*from\s*["'](.+)["']"###).unwrap();
    let export_matches = export_re
        .find_iter(&text)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<_>>();
    for export in export_matches.iter() {
        let export = export.replace('\n', " ");
        let captures = export_reference_re
            .captures(&export)
            .unwrap_or_else(|| panic!("unable to capture `{export}`"));
        let what = captures[1].to_string();
        let location = captures[2].to_string();
        let export = Reference::new(ReferenceKind::Export, referrer, Some(&what), &location);
        references.push(export);
    }
    let text = export_re.replace_all(&text, "");

    // let mut links = Vec::new();
    let link_re = Regex::new(
        r###"<link\s+rel=["']stylesheet["']\s+(type=["'][^"']+["']\s+)?href=["'][^"]+["']\s*/?>"###,
    )
    .unwrap();
    let link_reference_re = Regex::new(r###"href=["']([^"']+)["']"###).unwrap();
    let link_matches = link_re
        .find_iter(&text)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<_>>();
    for link in link_matches.iter() {
        let link = link.replace('\n', " ");
        let captures = link_reference_re
            .captures(&link)
            .unwrap_or_else(|| panic!("unable to capture `{link}`"));
        let location = captures[1].to_string();
        let import = Reference::new(ReferenceKind::Style, referrer, None, &location);
        references.push(import);
    }
    let text = link_re.replace_all(&text, "");

    // ~~~~~~~~~~~~~~~~~
    // FlowQRCode.define('flow-qrcode', [baseUrl+'resources/extern/qrcode/qrcode.js']);

    let text = {
        let mut text = text.to_string();
        // let define_re = Regex::new(r###"\w+.define\(["'][^"']+["'],\s*(\[[^]]+\])?\)"###).unwrap();
        let define_re =
            Regex::new(r###"\w+\.define\(["'][^"']+["'],\s*["'\[]([^]]+\])?"###).unwrap();
        let define_list_re = Regex::new(r###"\[([^\]]+)]"###).unwrap();
        let define_replace_re = Regex::new(r###"(,\s*\[([^\]]+)]\s*)"###).unwrap();
        let define_reference_re = Regex::new(r###"["']([^"']+)["']"###).unwrap();
        let define_matches = define_re
            .find_iter(&text)
            .map(|m| m.as_str().to_string())
            .collect::<Vec<_>>();
        for define in define_matches.iter() {
            // println!("A:{}",define);
            let replace = define_replace_re
                .captures(define)
                .unwrap_or_else(|| panic!("unable to capture replace for `{define}`"));
            text = text.replace(&replace[1].to_string(), "");
            let define = define.replace('\n', "");
            let captures = define_list_re
                .captures(&define)
                .unwrap_or_else(|| panic!("unable to capture list `{define}`"));
            let define = captures[1].to_string();
            let items = define.split(',').collect::<Vec<_>>();
            let items = items
                .iter()
                .filter_map(|s| {
                    // println!("capturing: `{}`",s);

                    define_reference_re
                        .captures(s)
                        .map(|captures| captures[1].to_string())

                    // match define_reference_re.captures(s) {
                    //     Some(captures) => Some(captures[1].to_string()),
                    //     None => None,
                    // }
                    // let captures = define_reference_re.captures(&s).expect(&format!("unable to capture item`{}`",define));
                    // captures[1].to_string()
                })
                // .flatten()
                .collect::<Vec<_>>();
            // println!("");

            for location in items.iter() {
                let extension = Path::new(location).extension().unwrap();
                let kind = if extension == "css" {
                    ReferenceKind::Style
                } else {
                    ReferenceKind::Script
                };

                let import = Reference::new(kind, referrer, None, location);
                references.push(import);
            }
        }

        text
    };

    let references = if references.is_empty() {
        None
    } else {
        Some(references)
    };

    Ok((references, text))
}

pub fn hex_str_to_u64(s: &str) -> Result<u64> {
    let without_prefix = s.trim_start_matches("0x");
    Ok(u64::from_str_radix(without_prefix, 16)?)
}

pub fn u64_to_hex_str(id: &u64) -> String {
    format!("0x{id:16x}")
}

pub trait GetHash {
    fn u64_hash(&self) -> u64;
}

impl GetHash for String {
    fn u64_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

// FlowQRCode.define('flow-qrcode', [baseUrl+'resources/extern/qrcode/qrcode.js']);

// pub fn strip_js_comments(source: &str) -> String {
//     let mut result = String::new();
//     let mut block_comment = false;

//     let mut chars = source.chars();
//     while let Some(c) = chars.next() {
//         if !block_comment {
//             if c == '/' {
//                 if let Some(next) = chars.next() {
//                     if next == '/' {
//                         // Skip single-line comment
//                         while let Some(c) = chars.next() {
//                             if c == '\n' {
//                                 break;
//                             }
//                         }
//                         continue;
//                     } else if next == '*' {
//                         // Start block comment
//                         block_comment = true;
//                         continue;
//                     } else {
//                         // Not a comment, add both characters to result
//                         result.push(c);
//                         result.push(next);
//                     }
//                 }
//             } else {
//                 // Not a comment, add character to result
//                 result.push(c);
//             }
//         } else {
//             if c == '*' {
//                 if let Some(next) = chars.next() {
//                     if next == '/' {
//                         // End block comment
//                         block_comment = false;
//                         continue;
//                     }
//                 }
//             }
//         }
//     }

//     result
// }
