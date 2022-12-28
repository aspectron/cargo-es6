use crate::prelude::*;


pub fn gather_references<P:AsRef<Path>>(kind: ReferenceKind, text: &str, referrer: P) -> Result<(Vec<Reference>, String)> {

    match kind {
        ReferenceKind::Import => {
            let mut imports = Vec::new();
            // handle `import xxx from "xxx"`
            let import_re = Regex::new(r###"import[^;]*from\s*["'][^"']+["'];"###).unwrap();
            let import_reference_re = Regex::new(r###"import\s*(.+)\s*from\s*["'](.+)["']"###).unwrap();
            let import_matches = import_re.find_iter(&text).map(|m| m.as_str().to_string()).collect::<Vec<_>>();
            for import in import_matches.iter() {
                let import = import.replace("\n"," ");
                let captures = import_reference_re.captures(&import).expect(&format!("unable to capture `{}`",import));
                let what = captures[1].to_string();
                let location = captures[2].to_string();
                let import = Reference::new(
                    ReferenceKind::Import,
                    referrer.as_ref(),
                    Some(&what),
                    &location
                );
                imports.push(import);
            }
            let text = import_re.replace_all(&text, "");
        
            // handle `import "..."
            // let import_re = Regex::new(r###"^\s*import\s*["'][^"']+["'];"###).unwrap();
            let import_re = Regex::new(r###"import\s*["'][^"']+["'];"###).unwrap();
            let import_reference_re = Regex::new(r###"import\s*["'](.+)["']"###).unwrap();
            let import_matches = import_re.find_iter(&text).map(|m| m.as_str().to_string()).collect::<Vec<_>>();
            for import in import_matches.iter() {
                // println!("| import len: {}", import.len());
                // println!("| import match: {} {}", import,import.len());
                let import = import.replace("\n"," ");
                let captures = import_reference_re.captures(&import).expect(&format!("unable to capture `{}`",import));
                // let what = "*".to_string();
                let location = captures[1].to_string();
                // println!("| import location: {}", location);
                let import = Reference::new(
                    ReferenceKind::Import,
                    referrer.as_ref(),
                    None,
                    &location
                );
                imports.push(import);
            }
            let text = import_re.replace_all(&text, "");
        
            Ok((imports, text.to_string()))
        },
        ReferenceKind::Export => {
            let mut exports = Vec::new();
            let export_re = Regex::new(r###"export[^;]*from\s*["']+[^"']+["']+;"###).unwrap();
            let export_reference_re = Regex::new(r###"export\s*(.+)\s*from\s*["'](.+)["']"###).unwrap();
            let export_matches = export_re.find_iter(&text).map(|m| m.as_str().to_string()).collect::<Vec<_>>();
            for export in export_matches.iter() {
                let export = export.replace("\n"," ");
                let captures = export_reference_re.captures(&export).expect(&format!("unable to capture `{}`",export));
                let what = captures[1].to_string();
                let location = captures[2].to_string();
                let export = Reference::new(
                    ReferenceKind::Export,
                    referrer.as_ref(),
                    Some(&what),
                    &location
                );
                exports.push(export);
            }
            let text = export_re.replace_all(&text, "");

            Ok((exports, text.to_string()))
        },
        _ => {
            panic!("gather_references() requires explicit reference type");
        }
    }

}

pub async fn gather_stylesheets<P : AsRef<Path>>(text : String, referrer: P) -> Result<(Vec<Reference>,String)> {

    // 		<link rel="stylesheet" href="${baseUrl}resources/extern/gridstack/gridstack.min.css">
    // let mut stylesheets = Vec::new();
    // handle `import xxx from "xxx"`
    // let import_re = Regex::new(r###"import[^;]*from\s*["'][^"']+["'];"###).unwrap();
    let mut links = Vec::new();
    let link_re = Regex::new(r###"<link\s+rel=["']stylesheet["']\s+href=["'][^"]+["']\s*>"###).unwrap();
    let link_reference_re = Regex::new(r###"href=["']([^"']+)["']"###).unwrap();
    let link_matches = link_re.find_iter(&text).map(|m| m.as_str().to_string()).collect::<Vec<_>>();
    for link in link_matches.iter() {
        let link = link.replace("\n"," ");
        let captures = link_reference_re.captures(&link).expect(&format!("unable to capture `{}`",link));
        let location = captures[1].to_string();
        let import = Reference::new(
            ReferenceKind::Style,
            referrer.as_ref(),
            None,
            &location
        );
        links.push(import);
    }
    let text = link_re.replace_all(&text, "");

    Ok((links, text.to_string()))
}