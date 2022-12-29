use crate::prelude::*;
use minify_js::*;

pub type ModuleId = u64;
static mut FILE_MODULE_GID: ModuleId = 0;

pub enum IdentKind {
    IntegerFull,
    HexFull,
    Integer,
    Hex,
}

#[derive(Debug, Clone)]
pub enum ContentType {
    Style,
    Module,
    Script,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Style => "ContentType::Style".to_string(),
            ContentType::Module => "ContentType::Module".to_string(),
            ContentType::Script => "ContentType::Script".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Content {
    pub id : ModuleId,
    pub ident : String,
    pub component : String,
    pub folder: PathBuf,
    // pub node_module_folder : Option<PathBuf>,
    pub base_folder : PathBuf,
    pub absolute : PathBuf,
    pub references : Option<Vec<Reference>>,
    // pub exports : Option<Vec<Reference>>,
    // pub styles : Option<Vec<Reference>>,
    // pub stylesheets : Vec<Reference>,
    pub content_type : ContentType,
    pub content : String,
}


impl Content {
    pub async fn load<P>(ctx: &Context, content_type:ContentType, folder :P, path: P) -> Result<Content> 
    where P: AsRef<Path> {
        let folder = folder.as_ref();
        let base_folder = folder.to_path_buf();
        
        let path = path.as_ref();
        let path_str = path.to_str().unwrap();
        let path_str = ctx.replace(path_str);
        let path = Path::new(&path_str);

        let absolute = folder.join(path);
        let parent = absolute.parent().unwrap();
        let component = Path::new(path).file_stem().unwrap().to_string_lossy().to_case(Case::Pascal);
        // println!("loading: `{}` -> `{}`",folder.display(),path.display());

        let extension = &path
            .extension()
            .expect(&format!("no extension in file: `{}`",path.display()))
            .to_str()
            .unwrap()
            .to_lowercase();
        // let content_type = match extension {

        // }

        // println!("{}",extension);

        let content_type = if extension == "css" {
            // println!("ContentType::Style {}", path.display());
            ContentType::Style
        } else {
            content_type
        };

        let text = async_std::fs::read_to_string(&absolute).await?;

        let text = match content_type {
            ContentType::Module | ContentType::Script => {
                let comment_re = Regex::new(r###"^\s*//"###).unwrap();
                text.split("\n").filter(|s| !comment_re.is_match(s) && !s.trim().is_empty()).collect::<Vec<_>>().join("\n")
            },
            _ => { text }
        };

        let (references, text) = gather_references(&text, &absolute)?;

        let references = if let Some(mut references) = references {

            for reference in references.iter_mut() {
                // let location = ctx
                reference.location = ctx.replace(&reference.location);
            }
            Some(references)
        } else {
            references
        };

        let ident_re_blank = Regex::new(r"(node_modules/|@|.js)")?;
        let ident_re_usc = Regex::new(r"/|-|\.")?;
        let ident = absolute.strip_prefix(&ctx.project_folder)?.to_string_lossy().to_string();
        let ident = ident_re_blank.replace_all(&ident,"")
            .to_string();
        let ident = ident_re_usc.replace_all(&ident, "_")
            .to_string()
            .to_uppercase();

        unsafe { FILE_MODULE_GID += 1 };
        let id = unsafe { FILE_MODULE_GID };

        let minify_content  = false;

        let content = if minify_content {
            match content_type {
                ContentType::Module => {
                    let mut content = Vec::new();
                    match minify(TopLevelMode::Module, text.as_bytes().to_vec(), &mut content) {
                        Ok(_) => {
                            String::from_utf8(content)?
                        },
                        Err(err) => {
                            println!("");
                            log_error!("minify in: `{}`", path.display());
                            log_error!("error: {}", err);
                            println!("");
                            text.to_string()
                        }
                    }
                },
                ContentType::Script => {
                    text.to_string()
                },
                _ => { text.to_string() }
            }
        } else {
            text.to_string()
        };

        let module = Content {
            id,
            ident,
            component,
            base_folder,
            folder: parent.to_path_buf(),
            absolute : absolute,
            references,
            content_type,
            content,
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

    pub fn references(&self) -> Vec<Arc<Content>> {
        let mut list = Vec::new();
        if let Some(references) = &self.references {
            for reference in references.iter() {
                if let Some(reference) = reference.content() {
                    list.push(reference);
                } else {
                    reference.warn();
                }
            }
        }
        list
    }

    pub fn gather(self : &Arc<Content>, collection: &mut Collection) -> Result<()> {

        self.gather_impl(collection)?;

        collection.modules.push(self.clone());
        collection.groups.push(vec![self.clone()]);

        Ok(())
    }

    fn gather_impl(self : &Arc<Content>, collection: &mut Collection) -> Result<()> {

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