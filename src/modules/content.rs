use crate::prelude::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub type ModuleId = u64;

pub enum IdentKind {
    Full,
    Integer,
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
    // pub folder: PathBuf,
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
    pub fn load<P>(ctx: &Context, content_type:ContentType, base_folder :P, path: P) -> Result<Content> 
    where P: AsRef<Path> {
        let base_folder = base_folder.as_ref().to_path_buf();
        let path = path.as_ref();
        let absolute = base_folder.join(path).canonicalize()?;
        let component = Path::new(path).file_stem().unwrap().to_string_lossy().to_case(Case::Pascal);
        let text = fs::read_to_string(&absolute)?;

        let text = match content_type {
            ContentType::Module | ContentType::Script => {

                // println!("processing: {}",absolute.display());
                // use no_comment::{IntoWithoutComments as _, languages};

                // let text = text
                // .chars()
                // .without_comments(languages::rust())
                // .collect::<String>();
                // ~~~
                // let text = strip_js_comments(text)?;
                // text.split("\n").map(|l|l.trim()).filter(|l|!l.is_empty()).collect::<Vec<_>>().join("\n")
                // text
                // let comment_block_re = Regex::new(r###"/\*[^(\*/)]*\*/"###).unwrap();
                // let text = comment_block_re.replace_all(&text, "");
                let comment_line_re = Regex::new(r###"^\s*//"###).unwrap();
                
                text.split("\n").filter(|s|{
                    if (comment_line_re.is_match(s) && !s.contains("*/")) || s.trim().is_empty() {
                        false
                    } else {
                        true
                    }
                    // !comment_line_re.is_match(s) && !s.trim().is_empty()
                }).collect::<Vec<_>>().join("\n")
                // text
            },
            _ => { 
                // println!("processing: {}",absolute.display());
                
                text }
        };

        // println!("processing: {}",absolute.display());
        let (references, text) = gather_references(&text, &absolute)?;
        let references = if let Some(mut references) = references {
            for reference in references.iter_mut() {
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


        let mut hasher = DefaultHasher::new();
        ident.hash(&mut hasher);
        let id = hasher.finish();

        let content = text.to_string();
        let module = Content {
            id,
            ident,
            component,
            base_folder,
            absolute,
            references,
            content_type,
            content,
        };

        Ok(module)

    }

    pub fn ident(&self, kind : &IdentKind) -> String {
        match kind {
            IdentKind::Full => {
                let content_type = match self.content_type {
                    ContentType::Module => "module",
                    ContentType::Script => "script",
                    ContentType::Style => "style",
                };
                format!("{}_{:x}_{}", content_type, self.id, self.ident).to_uppercase()
                // format!("{}_{:x}", self.ident, self.id).to_uppercase()
            },
            IdentKind::Integer => format!("_{:x}", self.id).to_uppercase(),
        }
    }

    pub fn id(&self) -> String {
        format!("0x{:016x}",self.id)
    }

    // pub fn id(&self, kind : &IdentKind) -> String {
    //     match kind {
    //         IdentKind::IntegerFull => format!("{}", self.id),
    //         IdentKind::Integer => format!("{}", self.id),
    //         IdentKind::HexFull => format!("{:x}", self.id).to_uppercase(),
    //         IdentKind::Hex => format!("{:x}", self.id).to_uppercase(),
    //     }
    // }

    // pub fn hex_id(&self) -> String {
    //     format!("{:x}", self.id)
    // }

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

        collection.content.push(self.clone());
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
                collection.content.push(reference.clone());
                group.push(reference.clone());
            }
        }

        if !group.is_empty() {
            collection.groups.push(group);
        }

        Ok(())
    }

    pub fn resolve(&self, db : &mut Db) -> Result<()> {
        if let Some(references) = &self.references {
            let references = references.iter().filter(|r|{
                r.reference.lock().unwrap().is_none()
            }).collect::<Vec<_>>();

            for reference in references.iter() {
                *reference.reference.lock().unwrap() = self.resolve_reference(db,reference)?;
            }
        }

        Ok(())
    }

    // pub async fn resolve(&mut self, location: &str, referrer : &Path) -> Result<Option<Arc<Content>>> {
    pub fn resolve_reference(&self, db : &mut Db, reference : &Reference) -> Result<Option<Arc<Content>>> {

        let location = &reference.location.clone();
        let location = if location.starts_with("./") {
            &location[2..]
        } else if location.starts_with("/") {
            &location[1..]
        } else {
            location
        };

        // check if location matches any node modules
        if let Some(node_module) = db.node_modules_by_name.get(location) {
            if let Some(module) = &node_module.root {
                return Ok(Some(module.clone()));
            }
        }

        // println!("--- relative: `{}`",location);
        let paths = [
            reference.referrer.parent().unwrap().join(location),
            self.absolute.parent().unwrap().join(location),
            self.base_folder.join(location),
            db.ctx.node_modules.join(location),
        ];

        let path = paths
            .iter()
            .find(|p|{
                p.canonicalize().is_ok()
            });

        if path.is_none() {
            reference.warn();
            let relative = self.absolute.strip_prefix(&db.ctx.project_folder)?;

            if !db.ctx.ignore.is_match(&relative.to_string_lossy()) && !db.ctx.ignore.is_match(&location) {
                println!("location: `{}`",location);
                reference.warn();
                return Err(format!("unable to resolve: `{}`", location).into())
            }
            return Ok(None);
        }

        let absolute = path.unwrap();

        if let Some(target) = db.file_content_by_absolute.get(absolute) {
            Ok(Some(target.clone()))
        } else {
            let content_type = match reference.kind {
                ReferenceKind::Module | ReferenceKind::Export => {
                    ContentType::Module
                },
                ReferenceKind::Script => {
                    ContentType::Script
                },
                ReferenceKind::Style => {
                    ContentType::Style
                }
            };

            // println!("creating ref content for: `{}`",reference.location);
            let base_folder = self.base_folder.clone();
            let dependency = Arc::new(Content::load(&db.ctx, content_type, base_folder, absolute.clone())?);
            db.file_content.push(dependency.clone());
            db.file_content_by_absolute.insert(absolute.clone(), dependency.clone());
            // *reference.reference.lock().unwrap() = Some(dependency.clone());
            dependency.resolve(db)?;
            Ok(Some(dependency))
        }

    }

}