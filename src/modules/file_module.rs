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
    pub component : String,
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
        let component = Path::new(path).file_stem().unwrap().to_string_lossy().to_case(Case::Pascal);
        // println!("loading: `{}` -> `{}`",folder.display(),path.display());


        let text = async_std::fs::read_to_string(&absolute).await?;

        let comment_re = Regex::new(r###"^\s*//"###).unwrap();
        let text = text.split("\n").filter(|s| !comment_re.is_match(s) && !s.trim().is_empty()).collect::<Vec<_>>().join("\n");

        let (imports, text) = gather_references(ReferenceKind::Import, &text, &absolute)?;
        let (exports, text) = gather_references(ReferenceKind::Export, &text, &absolute)?;

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
            component,
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