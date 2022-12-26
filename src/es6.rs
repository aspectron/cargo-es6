use crate::prelude::*;

// pub enum Reference {
//     FileModule(Arc<Module>),
//     NodeModule(Arc<NodeModule>),
// }

#[derive(Debug)]
pub struct Import {
    // pub origin: PathBuf,
    pub referrer: PathBuf,
    pub what: String,
    pub location: String,
    pub reference: Mutex<Option<Arc<FileModule>>>,
}

impl Import {
    pub fn new(referrer: &Path, what: &str, location: &str) -> Import {
        Import {
            referrer: referrer.to_path_buf(),
            what: what.to_string(),
            location: location.to_string(),
            reference: Mutex::new(None),
        }
    }
}

#[derive(Debug)]
pub struct Export {
    // pub origin: PathBuf,
    pub referrer: PathBuf,
    pub what: String,
    pub location: String,
    pub reference: Mutex<Option<Arc<FileModule>>>,
}

impl Export {
    pub fn new(referrer: &Path, what: &str, location: &str) -> Export {
        Export {
            referrer: referrer.to_path_buf(),
            what: what.to_string(),
            location: location.to_string(),
            reference: Mutex::new(None),
        }
    }
}

#[derive(Debug)]
pub struct FileModule {
    pub folder: PathBuf,
    pub absolute : PathBuf,
    pub imports : Vec<Import>,
    pub exports : Vec<Export>,
    pub content : String,
}


impl FileModule {
    pub async fn load<P>(folder :P, path: P) -> Result<FileModule> 
    where P: AsRef<Path> {
        let folder = folder.as_ref();
        let path = path.as_ref();
        let absolute = folder.join(path);
        let parent = absolute.parent().unwrap();
        // println!("loading: `{}` -> `{}`",folder.display(),path.display());

        let text = async_std::fs::read_to_string(&absolute).await?;

        let import_re = Regex::new(r###"import[^;]*from\s*["']+[^"']+["']+;"###).unwrap();
        let import_reference_re = Regex::new(r###"import\s*(.+)\s*from\s*["'](.+)["']"###).unwrap();
        let import_matches = import_re.find_iter(&text).map(|m| m.as_str().to_string()).collect::<Vec<_>>();
        let mut imports = Vec::new();
        for import in import_matches.iter() {
            let import = import.replace("\n"," ");
            let captures = import_reference_re.captures(&import).expect(&format!("unable to capture `{}`",import));
            let what = captures[1].to_string();
            let location = captures[2].to_string();
            let import = Import::new(
                &absolute,
                &what,
                &location
            );
            imports.push(import);
        }
        let text = import_re.replace_all(&text, "");

        let export_re = Regex::new(r###"export[^;]*from\s*["']+[^"']+["']+;"###).unwrap();
        let export_reference_re = Regex::new(r###"export\s*(.+)\s*from\s*["'](.+)["']"###).unwrap();
        let export_matches = export_re.find_iter(&text).map(|m| m.as_str().to_string()).collect::<Vec<_>>();
        let mut exports = Vec::new();
        for export in export_matches.iter() {
            let export = export.replace("\n"," ");
            let captures = export_reference_re.captures(&export).expect(&format!("unable to capture `{}`",export));
            let what = captures[1].to_string();
            let location = captures[2].to_string();
            let export = Export::new(
                &absolute,
                &what,
                &location
            );
            exports.push(export);
        }
        let text = export_re.replace_all(&text, "");

        let module = FileModule {
            folder: parent.to_path_buf(), //folder.to_path_buf(),
            absolute : absolute,// path.to_path_buf(),
            imports,
            exports,
            content: text.to_string(),
        };

        Ok(module)

    }
}