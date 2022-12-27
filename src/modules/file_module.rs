use crate::prelude::*;

pub type ModuleId = u64;
static mut FILE_MODULE_GID: ModuleId = 0;

pub enum IdentKind {
    IntegerFull,
    HexFull,
    Integer,
    Hex,
}

#[derive(Debug)]
pub struct FileModule {
    pub id : ModuleId,
    pub ident : String,
    pub folder: PathBuf,
    pub absolute : PathBuf,
    pub imports : Vec<Reference>,
    pub exports : Vec<Reference>,
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

        let comment_re = Regex::new(r###"^\s*//"###).unwrap();
        let text = text.split("\n").filter(|s| !comment_re.is_match(s) && !s.trim().is_empty()).collect::<Vec<_>>().join("\n");

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
                &absolute,
                &what,
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
            let what = "*".to_string();
            let location = captures[1].to_string();
            // println!("| import location: {}", location);
            let import = Reference::new(
                ReferenceKind::ImportAll,
                &absolute,
                &what,
                &location
            );
            imports.push(import);
        }
        let text = import_re.replace_all(&text, "");

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
                &absolute,
                &what,
                &location
            );
            exports.push(export);
        }
        let text = export_re.replace_all(&text, "");

        let ident_re_blank = Regex::new(r"(node_modules/|@|.js)")?;
        let ident_re_usc = Regex::new(r"/|-|\.")?;
        let ident = absolute.strip_prefix(&ctx.project_folder)?.to_string_lossy().to_string();
        // let ident = ident_path
        //     .replace("node_modules/","");
        let ident = ident_re_blank.replace_all(&ident,"")
            .to_string();
        let ident = ident_re_usc.replace_all(&ident, "_")
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

    pub fn ident(&self, kind : &IdentKind) -> String {
        match kind {
            IdentKind::IntegerFull => format!("{}_{}", self.ident, self.id),
            IdentKind::Integer => format!("_{}", self.id),
            IdentKind::HexFull => format!("{}_{:x}", self.ident, self.id).to_uppercase(),
            IdentKind::Hex => format!("_{:x}", self.id).to_uppercase(),
        }
    }

    pub fn id(&self, kind : &IdentKind) -> String {
        match kind {
            IdentKind::IntegerFull => format!("{}", self.id),
            IdentKind::Integer => format!("{}", self.id),
            IdentKind::HexFull => format!("{:x}", self.id).to_uppercase(),
            IdentKind::Hex => format!("{:x}", self.id).to_uppercase(),
        }
    }

    pub fn hex_id(&self) -> String {
        format!("{:x}", self.id)
    }

    pub fn references(&self) -> Vec<Arc<FileModule>> {
        let mut list = Vec::new();
        for import in self.imports.iter() {
            if let Some(reference) = import.reference() {
                list.push(reference);
            } else {
                import.warn();
            }
        }
        for export in self.exports.iter() {
            if let Some(reference) = export.reference() {
                list.push(reference);
            } else {
                export.warn();
            }
        }
        list
    }

    pub fn gather(self : &Arc<FileModule>, collection: &mut Collection) -> Result<()> {

        self.gather_impl(collection)?;

        collection.modules.push(self.clone());
        collection.groups.push(vec![self.clone()]);

        Ok(())
    }

    fn gather_impl(self : &Arc<FileModule>, collection: &mut Collection) -> Result<()> {

        let mut group = Vec::new();
        let references = self.references();

        for reference in references.iter() {
            reference.gather_impl(collection)?;
        }

        for reference in references.iter() {
            if !collection.idents.contains(&reference.ident) {
                collection.idents.insert(reference.ident.clone());
                collection.modules.push(reference.clone());
                group.push(reference.clone());
            }
        }

        if !group.is_empty() {
            collection.groups.push(group);
        }

        Ok(())
    }
}