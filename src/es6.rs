use crate::prelude::*;

pub struct Import {
    what: String,
    location: String,
    // reference: Mutex< 
}

pub struct Module {
    pub folder: PathBuf,
    pub path : PathBuf,
    pub imports : Vec<Import>,
    pub content : String,
}


impl Module {
    pub async fn load<P>(folder :P, path: P) -> Result<Module> 
    where P: AsRef<Path> {
        let folder = folder.as_ref();
        let path = path.as_ref();

        println!("loading: `{}` -> `{}`",folder.display(),path.display());

        let text = async_std::fs::read_to_string(folder.join(path)).await?;

        let import_re = Regex::new(r###"import[^;]*from\s*["']+[^"']+["']+;"###).unwrap();
        let import_reference_re = Regex::new(r###"import\s*(.+)\s*from\s*["'](.+)["']"###).unwrap();
        let import_matches = import_re.find_iter(&text).map(|m| m.as_str().to_string()).collect::<Vec<_>>();

        // println!("{}:",absolute.display());
        // println!("{:#?}",import_matches);
        let mut imports = Vec::new();
        for import in import_matches.iter() {
            let import = import.replace("\n"," ");
            let captures = import_reference_re.captures(&import).expect(&format!("unable to capture `{}`",import));
            let what = captures[1].to_string();
            let location = captures[2].to_string();
            // println!("relative: {}",location);
            // let folder = if filename.starts_with("/") {
            //     project_folder.clone()
            // } else {
            //     // project_folder.join(file).to_owned()
            //     absolute.parent().unwrap().to_path_buf()
            // };
            // let filename = if filename.starts_with("./") {
            //     filename[2..].to_string()
            // } else { filename };
            // let import = match folder.join(&filename).canonicalize().await {
            //     Ok(path) => {
            //         println!("absolute: {}",path.display());
            //         Import { what, from : Reference::File(path.to_str().unwrap().to_string()) }
            //     }, 
            //     Err(e) => {
            //         println!("error: {}",style(e).red());
            //         Import { what, from : Reference::Module(filename) }
            //     }
            // };

            imports.push(Import { what, location });
            // let filename = .expect(&format!("unable to canonicalize {}",filename));
        }
        let text = import_re.replace_all(&text, "");

        let module = Module {
            folder: folder.to_path_buf(),
            path : path.to_path_buf(),
            imports,
            content: text.to_string(),
        };

        Ok(module)

    }
}