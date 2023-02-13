use crate::prelude::*;

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

impl ContentType {
    pub fn to_manifest_type(&self) -> String {
        match self {
            ContentType::Style => "style".to_string(),
            ContentType::Module => "module".to_string(),
            ContentType::Script => "script".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Content {
    pub id: ModuleId,
    pub ident: String,
    pub component: String,
    /// node_modules folder in which the file resides (if aailable)
    /// if not available, this contains the project folder
    // pub base_folder : PathBuf,
    pub node_module_id: u64,
    pub location: PathBuf,
    // pub absolute: PathBuf,
    pub references: Option<Vec<Reference>>,
    pub content_type: ContentType,
    pub contents: Option<String>,
    pub external: bool,
}

impl Content {
    // pub fn load<P>(ctx: &Context, content_type:ContentType, base_folder :P, absolute: P) -> Result<Content>
    // pub fn load<P>(ctx: &Context, content_type:ContentType, node_module : &NodeModule, location: P) -> Result<Content>
    pub fn get_ident<P: AsRef<Path>>(ctx: &Context, p: P) -> Result<String> {
        let ident_re_blank = Regex::new(r"(node_modules/|@|.js)")?;
        let ident_re_usc = Regex::new(r"/|-|\.")?;
        // let ident = p.as_ref().strip_prefix(&ctx.project_folder)?.to_string_lossy().to_string();
        let ident = p.as_ref().to_string_lossy().to_string();
        let ident = ident_re_blank.replace_all(&ident, "").to_string();
        let ident = ident_re_usc
            .replace_all(&ident, "_")
            .to_string()
            .to_uppercase();
        Ok(ident)
    }

    pub fn load<P>(
        // ctx: &Context,
        db: &mut Db,
        content_type: ContentType,
        // node_module : &NodeModule,
        node_module_id: u64,
        base_folder: P,
        location: P,
    ) -> Result<Content>
    where
        P: AsRef<Path>,
    {
        let location = location.as_ref().to_path_buf();
        // println!("loading base_folder: {}", base_folder.as_ref().display());
        // println!("loading location: {}", location.display());
        let absolute = db
            .ctx
            .project_folder
            .join(&base_folder)
            .join(&location)
            .canonicalize()
            .unwrap_or_else(|err|panic!("unable to canonicalize to absolute {err}"));
        let ident = Self::get_ident(&db.ctx, &absolute)?;
        let id = ident.u64_hash();
        // println!("loading absolute: {}", absolute.display());

        // if let Some(content) = db.get_file(&id) {
        //     return Ok(content.clone());
        // }

        let text = fs::read_to_string(&absolute)?;
        let text = match content_type {
            ContentType::Module | ContentType::Script => {
                let comment_line_re = Regex::new(r###"^\s*//"###).unwrap();
                text.split("\n")
                    .filter(|s| {
                        if (comment_line_re.is_match(s) && !s.contains("*/")) || s.trim().is_empty()
                        {
                            false
                        } else {
                            true
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            _ => text,
        };

        // println!("processing: {}",absolute.display());
        let (references, text) = gather_references(&text, id)?;
        let references = if let Some(mut references) = references {
            for reference in references.iter_mut() {
                reference.location = db.ctx.replace(&reference.location);
            }
            Some(references)
        } else {
            references
        };

        let contents = Some(text.to_string());

        let content = Content::new(
            db,
            content_type,
            node_module_id,
            location,
            id,
            contents,
            references,
        )?;

        // let content = Arc::new(content);
        // db.insert_file(&content);

        Ok(content)
    }

    // pub fn external(ctx: &Context, content_type:ContentType, id: u64, reference: &ExternalContentReference) -> Result<Content> {
    pub fn external(
        db: &mut Db,
        content_type: ContentType,
        id: u64,
        node_module_id: u64,
        location: &str,
    ) -> Result<Content> {
        // let base_folder = ctx.project_folder.join(&reference.base);
        // let absolute = base_folder.join(&reference.location);

        // let base_folder = base_folder.as_ref().to_path_buf();
        // let path = path.as_ref();
        // let absolute = base_folder.join(path).canonicalize()?;
        // let component = absolute.file_stem().unwrap().to_string_lossy().to_case(Case::Pascal);

        Content::new(db, content_type, node_module_id, location, id, None, None)
        // let content = text.to_string();
        // let module = Content {
        //     id,
        //     ident,
        //     component,
        //     base_folder,
        //     absolute,
        //     references : None,
        //     content_type,
        //     content : None,
        // };

        // Ok(module)
    }
    fn new<P>(
        // ctx: &Context,
        db: &mut Db,
        content_type: ContentType,
        node_module_id: u64,
        // base_folder :P,
        location: P,
        id: u64,
        contents: Option<String>,
        references: Option<Vec<Reference>>,
    ) -> Result<Content>
    where
        P: AsRef<Path>,
    {
        let location = location.as_ref().to_path_buf();
        let component = location
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_case(Case::Pascal);
        let ident = Self::get_ident(&db.ctx, &location)?;

        let external = contents.is_some();
        let module = Content {
            id,
            ident,
            component,
            // base_folder,
            node_module_id,
            location,
            references,
            content_type,
            contents,
            external,
        };

        Ok(module)
    }

    // pub fn get_export_path()

    pub fn ident(&self, kind: &IdentKind) -> String {
        match kind {
            IdentKind::Full => {
                let content_type = match self.content_type {
                    ContentType::Module => "module",
                    ContentType::Script => "script",
                    ContentType::Style => "style",
                };
                format!("{}_{:x}_{}", content_type, self.id, self.ident).to_uppercase()
                // format!("{}_{:x}", self.ident, self.id).to_uppercase()
            }
            IdentKind::Integer => format!("_{:x}", self.id).to_uppercase(),
        }
    }

    pub fn id(&self) -> String {
        format!("0x{:016x}", self.id)
    }

    pub fn get_absolute_path(&self, db: &Db) -> Result<PathBuf> {
        // Ok(ctx.project_folder.join(&self.location))
        let node_module = db.get_node_module(&self.node_module_id).unwrap();
        let absolute = node_module
            .get_absolute_path(&db)
            .join(&self.location)
            .canonicalize()
            .expect(&format!("unable to canonicalize to absolute"));

        // db
        //     .ctx
        //     .project_folder
        //     .join(&node_module.base_folder)
        //     .join(&self.location)
        Ok(absolute)
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

    pub fn references(&self, db: &Db) -> Result<Option<Vec<Arc<Content>>>> {
        // let references = self.references.lock()?;
        if let Some(references) = &self.references {
            let mut list = Vec::new();
            for reference in references.iter() {
                if let Some(reference) = reference.content() {
                    list.push(reference);
                } else {
                    reference.warn(db);
                }
            }
            Ok(Some(list))
        } else {
            Ok(None)
        }
    }

    pub fn gather(self: &Arc<Content>, db: &Db, collection: &mut Collection) -> Result<()> {
        self.gather_impl(db, collection)?;

        collection.content.push(self.clone());
        collection.groups.push(vec![self.clone()]);

        Ok(())
    }

    fn gather_impl(self: &Arc<Content>, db: &Db, collection: &mut Collection) -> Result<()> {
        let mut group = Vec::new();
        let references = self.references(db)?;
        if let Some(references) = references {
            for reference in references.iter() {
                reference.gather_impl(db, collection)?;
            }

            for reference in references.iter() {
                if !collection.idents.contains(&reference.ident) {
                    collection.idents.insert(reference.ident.clone());
                    collection.content.push(reference.clone());
                    group.push(reference.clone());
                }
            }
        }

        if !group.is_empty() {
            collection.groups.push(group);
        }

        Ok(())
    }

    pub fn node_module(&self, db: &Db) -> Arc<NodeModule> {
        db.node_modules_by_id
            .get(&self.node_module_id)
            .expect(&format!(
                "Unable to location node module in `{}`",
                self.location.display()
            ))
            .clone()
    }

    pub fn unresolved_references(&self) -> Option<Vec<&Reference>> {
        if let Some(references) = &self.references {
            let references = references
                .iter()
                .filter(|r| r.content.lock().unwrap().is_none())
                .collect::<Vec<_>>();
            if references.is_empty() {
                None
            } else {
                Some(references)
            }
        } else {
            None
        }
    }

    // pub fn resolve(self : &Arc<Self>, db : &mut Db) -> Result<()> {
    //     if let Some(references) = &self.references {
    //         let references = references.iter().filter(|r|{
    //             r.content.lock().unwrap().is_none()
    //         }).collect::<Vec<_>>();

    //         for reference in references.iter() {
    //             *reference.content.lock().unwrap() = Some(db.resolve(reference)?);
    //             // *reference.content.lock().unwrap() = self.resolve_reference(db,reference)?;
    //         }
    //     }

    //     Ok(())
    // }

    // pub async fn resolve(&mut self, location: &str, referrer : &Path) -> Result<Option<Arc<Content>>> {
    pub fn resolve_reference(
        self: &Arc<Self>,
        db: &mut Db,
        reference: &Reference,
    ) -> Result<Option<Arc<Content>>> {
        println!("dependency processing location: {}", reference.location);

        let location = Path::new(&reference.location).normalize()?;
        let location = location.to_str().unwrap();
        println!("normalized location: {}", location);

        // let location = &reference.location.clone();
        // let location = if location.starts_with("./") {
        //     &location[2..]
        // } else if location.starts_with("/") {
        //     &location[1..]
        // } else {
        //     location
        // };

        let content = db.locate(location, self)?;
        if content.is_some() {
            return Ok(content);
        }

        // // println!("--- relative: `{}`",location);
        // let paths = [
        //     reference.referrer.parent().unwrap().join(location),
        //     self.location.parent().unwrap().join(location),
        //     self.base_folder.join(location),
        //     // db.ctx.node_modules.join(location),
        // ];

        // let path = paths
        //     .iter()
        //     .find(|p|{
        //         p.canonicalize().is_ok()
        //     });

        // let (base_folder, path) = if path.is_some() {
        //     (Some(self.base_folder.clone()), path)
        // } else {
        //     db.locate_file(location)
        //     // let absolute = db.ctx.node_modules.join(location);
        //     // let folder = absolute.parent().unwrap();
        //     // let package_json = search_upwards(folder, "package.json").unwrap();
        //     // db.locate_file_in_node_modules(location)
        // };

        // if path.is_none() {
        //     reference.warn();
        //     let relative = self.location.strip_prefix(&db.ctx.project_folder)?;

        //     if !db.ctx.ignore.is_match(&relative.to_string_lossy()) && !db.ctx.ignore.is_match(&location) {
        //         println!("location: `{}`",location);
        //         reference.warn();
        //         return Err(format!("unable to resolve: `{}`", location).into())
        //     }
        //     return Ok(None);
        // }

        //

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        // let absolute = path.unwrap();

        // if let Some(target) = db.locate_file_absolute(absolute) {
        //     Ok(Some(target.clone()))
        // } else {
        //     let content_type = match reference.kind {
        //         ReferenceKind::Module | ReferenceKind::Export => {
        //             ContentType::Module
        //         },
        //         ReferenceKind::Script => {
        //             ContentType::Script
        //         },
        //         ReferenceKind::Style => {
        //             ContentType::Style
        //         }
        //     };

        //     // println!("creating ref content for: `{}`",reference.location);
        //     let base_folder = self.base_folder.clone();
        //     println!("loading dependency");
        //     let dependency = Arc::new(Content::load(&db.ctx, content_type, base_folder, absolute.clone())?);
        //     db.file_content.push(dependency.clone());
        //     db.file_content_by_location.insert(absolute.clone(), dependency.clone());
        //     // *reference.reference.lock().unwrap() = Some(dependency.clone());
        //     dependency.resolve(db)?;
        //     Ok(Some(dependency))
        // }
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        Err(format!(
            "Unable to resolve reference `{}` from `{}`",
            reference.location,
            self.location.display()
        )
        .into())
    }
}
