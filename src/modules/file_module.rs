use crate::prelude::*;

static mut FILE_MODULE_GID: u32 = 0;

#[derive(Debug)]
pub struct FileModule {
    pub id : u32,
    pub ident : String,
    pub folder: PathBuf,
    pub absolute : PathBuf,
    pub imports : Vec<Import>,
    pub exports : Vec<Export>,
    pub content : String,
}


impl FileModule {
    pub async fn load<P>(ctx: &Context, folder :P, path: P) -> Result<FileModule> 
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

        let ident_re_usc = Regex::new(r"[/-]")?;
        let ident_re_blank = Regex::new(r"(node_modules_|@|.js)")?;
        let ident = absolute.strip_prefix(&ctx.project_folder)?.to_string_lossy().to_string();
        // let ident = ident_path
        //     .replace("node_modules/","");
        let ident = ident_re_usc.replace_all(&ident, "_")
            .to_string();
        let ident = ident_re_blank.replace_all(&ident,"")
            .to_string()
            .to_uppercase();

        // println!("{}",ident);

        unsafe { FILE_MODULE_GID += 1 };
        let id = unsafe { FILE_MODULE_GID };

        let module = FileModule {
            id,
            ident,
            folder: parent.to_path_buf(), //folder.to_path_buf(),
            absolute : absolute,// path.to_path_buf(),
            imports,
            exports,
            content: text.to_string(),
        };

        Ok(module)

    }

    pub fn gather(self : &Arc<FileModule>, collection: &mut Collection) -> Result<()> {

        if collection.idents.contains(&self.ident) {
            // log_warn!("Cyclic","skipping duplicate reference ...");
            return Ok(());
        } else {
            collection.idents.insert(self.ident.clone());
        }

        for import in self.imports.iter() {
            if let Some(reference) = import.reference() {
                reference.gather(collection)?;
            } else {
                import.warn();
            }
        }

        for export in self.exports.iter() {
            if let Some(reference) = export.reference() {
                reference.gather(collection)?;
            } else {
                export.warn();
            }
        }

        collection.modules.push(self.clone());

        Ok(())
    }
}