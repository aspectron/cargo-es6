use crate::prelude::*;

pub struct FileContext {
    pub base_folder: PathBuf,
    pub absolute: PathBuf,
}

impl FileContext {
    pub fn new(base_folder: PathBuf, absolute: PathBuf) -> Self {
        FileContext {
            base_folder,
            absolute,
        }
    }
}
// impl From<(&Arc<NodeModule>, )

pub struct Dependencies {
    pub file_content: HashMap<String, Arc<Content>>,
    pub node_modules: HashMap<String, Arc<Content>>,
}
impl Dependencies {
    pub fn new() -> Dependencies {
        Dependencies {
            file_content: HashMap::new(),
            node_modules: HashMap::new(),
        }
    }
}

pub struct Db {
    pub ctx: Arc<Context>,
    // pub project_folder : PathBuf,
    pub node_modules: Vec<Arc<NodeModule>>,
    pub node_modules_by_id: HashMap<u64, Arc<NodeModule>>,
    pub node_modules_by_name: HashMap<String, Arc<NodeModule>>,
    pub node_modules_by_location: HashMap<PathBuf, Arc<NodeModule>>,
    // pub node_modules_by_absolute : HashMap<PathBuf, Arc<NodeModule>>,
    pub file_content: Vec<Arc<Content>>,
    pub file_content_by_id: HashMap<u64, Arc<Content>>,
    pub file_content_by_location: HashMap<PathBuf, Arc<Content>>,
    // pub file_content_by_absolute : HashMap<PathBuf, Arc<Content>>,
    // pub dependencies : Dependencies,

    pub project_module: Option<Arc<NodeModule>>,
    pub project_main: Option<Arc<Content>>,

}

impl Db {
    pub fn get_file(&self, id: &u64) -> Option<&Arc<Content>> {
        self.file_content_by_id.get(id)
    }

    pub fn get_node_module(&self, id: &u64) -> Option<&Arc<NodeModule>> {
        self.node_modules_by_id.get(id)
    }

    pub fn get_node_module_by_name(&self, name: &str) -> Option<&Arc<NodeModule>> {
        self.node_modules_by_name.get(name)
    }

    pub fn insert_file(&mut self, content: &Arc<Content>) -> Result<()> {
        self.file_content.push(content.clone());
        self.file_content_by_id.insert(content.id, content.clone());
        let location = self
            .get_node_module(&content.node_module_id)
            .expect(&format!(
                "Unable to locate node module for `{}`",
                content.location.display()
            ))
            .base_folder()
            .join(&content.location);
        // println!("inserting file: {}", location.display());
        // self.file_content_by_location.insert(content.location.clone(), content.clone());
        self.file_content_by_location
            .insert(location, content.clone());
        Ok(())
    }

    // pub fn register_node_module(&mut self, node_module: &Arc<NodeModule>) {
    //     // self.node_modules_by_absolute.insert(node_module.absolute.clone(),node_module.clone());

    // }

    // pub fn locate_file_absolute(&self, path : &PathBuf) -> Option<&Arc<Content>> {
    //     self.file_content_by_absolute.get(path)
    // }

    // pub fn find_node_module(&self, path : &PathBuf) -> Option<&Arc<Content>> {
    //     self.node.get(path)
    // }

    pub fn new(ctx: &Arc<Context>) -> Db {
        Db {
            ctx: ctx.clone(),
            node_modules: Vec::new(),
            node_modules_by_id: HashMap::new(),
            node_modules_by_name: HashMap::new(),
            node_modules_by_location: HashMap::new(),
            // node_modules_by_absolute : HashMap::new(),
            file_content: Vec::new(),
            file_content_by_id: HashMap::new(),
            file_content_by_location: HashMap::new(),

            project_module: None, //Option<Arc<NodeModule>,
            project_main: None,
            // dependencies
        }
    }

    pub fn insert_node_module(&mut self, node_module: &Arc<NodeModule>) -> Result<()> {
        // self.node_modules_by_absolute.insert(node_module.base_folder().clone(), node_module.clone());
        self.node_modules_by_location
            .insert(node_module.base_folder().clone(), node_module.clone());

        // for file in node_module.files.iter() {
        //     self.insert_file(&file)?;
        // }

        // // TODO
        // for (target,exports) in node_module.exports.iter() {
        //     for (name, content) in exports.iter() {
        //         self.insert_file(content)?;
        //     }
        // }

        self.node_modules.push(node_module.clone());
        self.node_modules_by_id
            .insert(node_module.id, node_module.clone());
        if node_module.name == node_module.alt_name {
            self.node_modules_by_name
                .insert(node_module.name.clone(), node_module.clone());
        } else {
            self.node_modules_by_name
                .insert(node_module.name.clone(), node_module.clone());
            self.node_modules_by_name
                .insert(node_module.alt_name.clone(), node_module.clone());
        }

        Ok(())
    }

    pub fn get_node_module_by_location(&self, location: &PathBuf) -> Option<&Arc<NodeModule>> {
        self.node_modules_by_location.get(location)
    }

    pub async fn load(ctx: &Arc<Context>) -> Result<Db> {
        let mut db = Db::new(ctx);

        log_info!("Loading","`{}`",ctx.project_folder.display());
        // load and resolve all dependencies for the main project node module
        let project_node_module = Arc::new(NodeModule::load(&mut db, &ctx.project_folder)?);
        db.project_module = Some(project_node_module.clone());
        db.insert_node_module(&project_node_module)?;
        log_info!("Resolving","`{}`",ctx.project_folder.display());
        let resolved_node_modules = db.resolve_dependencies(&project_node_module)?;
        
        log_info!("Deps","`{}`",ctx.project_folder.display());
        // iterate over all node module dependencies and resolve references
        for node_module in resolved_node_modules.iter() {
            // log_info!("Module","{}",node_module.name);
            if let Some(location) = node_module.main_file_relative(&db.ctx)? {
                let content = Arc::new(Content::load(
                    &mut db,
                    ContentType::Module,
                    node_module.id,
                    node_module.base_folder(),
                    &location,
                )?);
        
                db.insert_file(&content)?;
                db.resolve_references(&content)?;
            }
        }

        
        let location = project_node_module
            .main_file_relative(&db.ctx)?
            .expect("Missing main file in the project module");

        log_info!("Main","`{}`",location.display());
        
        let content = Arc::new(Content::load(
            &mut db,
            ContentType::Module,
            project_node_module.id,
            project_node_module.base_folder(),
            &location,
        )?);

        db.insert_file(&content)?;
        db.resolve_references(&content)?;
        db.project_main = Some(content);


        Ok(db)
    }

    pub fn resolve_dependencies(&mut self, node_module: &NodeModule) -> Result<Vec<Arc<NodeModule>>> {
        let mut resolved = vec![];
        if let Some(dependencies) = node_module.dependencies() {
            for dependency in dependencies {
                if self
                    .get_node_module_by_location(&PathBuf::from(&dependency))
                    .is_none()
                {
                    resolved.extend(self.resolve_dependency(node_module, dependency)?);
                }
            }
        }

        Ok(resolved)
    }

    pub fn resolve_dependency(&mut self, referrer: &NodeModule, dependency: &str) -> Result<Vec<Arc<NodeModule>>> {

        log_info!("Resolving","`{}`",dependency);
        // println!(
        //     "resolving dependency: {} -> {}",
        //     referrer.package_json.name.as_ref().unwrap(),
        //     dependency
        // );

        let mut list = vec![];
        let absolute_folder = self.ctx.node_modules.join(dependency);
        let location = absolute_folder
            .strip_prefix(&self.ctx.project_folder)?
            .to_path_buf();
        if self.get_node_module_by_location(&location).is_none() {
            if !absolute_folder.is_dir() {
                return Err(format!(
                    "Unable to resolve {} while processing {}",
                    dependency,
                    referrer.package_json.name.as_ref().unwrap()
                )
                .into());
            }

            // println!("loading {}", dependency);
            let node_module = Arc::new(NodeModule::load(self, &absolute_folder)?);
            self.insert_node_module(&node_module)?;
            list.push(node_module.clone());
        }

        let mut resolved = vec![];
        for node_module in list.iter() {
            resolved.push(node_module.clone());
            resolved.extend(self.resolve_dependencies(node_module)?);
        }

        Ok(resolved)
    }

    pub fn resolve_references(&mut self, content: &Content) -> Result<()> {
        if let Some(references) = content.unresolved_references() {
            let mut list = vec![];

            for reference in references {
                let resolved = self.resolve(reference)?;
                *reference.content.lock().unwrap() = Some(resolved.clone());
                list.push(resolved);
            }

            // for resolved in list.iter() {
            //     self.resolve_references(resolved)?;
            // }
        }


        Ok(())
    }

    pub fn resolve(&mut self, reference: &Reference) -> Result<Arc<Content>> {
        let Reference {
            kind,
            referrer,
            what,
            location,
            content,
        } = reference;

        let referrer = self.get_file(referrer).unwrap().clone();
        let referrer_node_module = referrer.node_module(self);
        // println!("{} :: {} -> {:?}", referrer_node_module.name, referrer.location.display(), reference.location);

        let content_type: ContentType = kind.into();

        let content =
            self.locate_content(content_type,&referrer_node_module, &referrer, location)?;

        Ok(content)
    }

    pub fn locate_or_create_existing(
        &mut self,
        content_type: ContentType,
        node_module: &NodeModule,
        absolute_path: &Path

    ) -> Result<Arc<Content>> {
        let project_relative = absolute_path.strip_prefix(&self.ctx.project_folder)?;

        // println!("looking for {}", project_relative.display());
        if let Some(content) = self.file_content_by_location.get(project_relative) {
            // println!("found cache for: `{}`",project_relative.display());
            return Ok(content.clone());
        }

        let node_module_absolute = node_module.get_absolute_path(self);
        let node_module_relative = absolute_path.strip_prefix(&node_module_absolute)?;

        let content = Arc::new(Content::load(
            self,
            content_type,
            node_module.id,
            node_module.base_folder(),
            &node_module_relative.to_path_buf()
        )?);

        self.insert_file(&content)?;

        self.resolve_references(&content)?;

        return Ok(content);
    }



    fn locate_content(
        &mut self,
        content_type: ContentType,
        referrer_node_module: &NodeModule,
        referrer_content: &Content,
        location: &String,
    ) -> Result<Arc<Content>> {

        let relative_regex = Regex::new(r"^\.\.?/").unwrap();
        if relative_regex.is_match(location) {

            let absolute = referrer_content.get_absolute_path(self)?;
            let parent = absolute.parent().ok_or_else(||format!("Unable to get parent of `{}`",absolute.display()))?;
            if let Ok(absolute_path) = parent.join(location).canonicalize() {
                let content = self.locate_or_create_existing(content_type, referrer_node_module, &absolute_path)?;
                return Ok(content) 
            }
        }

        let location = Path::new(location);
        let relative_location = if location.starts_with("/") {
            location.strip_prefix("/").unwrap()
        } else {
            location
        };

        let absolute = referrer_node_module.get_absolute_path(self);
        if let Ok(absolute_path) = absolute.join(relative_location).canonicalize() {

            let content = self.locate_or_create_existing(content_type, referrer_node_module,
                &absolute_path)?;
            return Ok(content) 
        }


        let mut parts = location.components().map(|c|c.as_os_str().to_str().unwrap()).collect::<Vec<_>>();
        if !parts.is_empty() {
            let node_module = if parts.get(0).unwrap().starts_with("@") {
                if parts.len() < 2 {
                    return Err(format!(
                        "Unable to resolve {} referred by {}",
                        location.display(),
                        // TODO - track absolute!!!
                        referrer_content.location.display()
                    ).into());

                }
                format!("{}/{}",parts.remove(0),parts.remove(0))
            } else {
                parts.remove(0).to_string()
            };

            let node_module_instance = self.get_node_module_by_name(&node_module).cloned();
            if let Some(node_module) = node_module_instance { //self.get_node_module_by_name(&node_module) {
                let location = if parts.is_empty() {
                    node_module.main_file_relative(&self.ctx)?.unwrap()
                } else {
                    PathBuf::from(parts.join("/"))
                };

                // let absolute_path = node_module.get_absolute_path(self).join(&location);
                // let absolute_path = node_module.resolve_exports_absolute(absolute_path);
                // if !absolute_path.is_file() {
                //     return Err(format!(
                //         "Unable to find {} in {}",
                //         location.display(),
                //         node_module.get_absolute_path(self).display()
                //     ).into());
                // }

                let absolute_path = node_module.resolve_absolute_content(self, location)?;

                let content = self.locate_or_create_existing(content_type, &node_module, &absolute_path)?;
                return Ok(content) 

            } else {
                println!("\n\n --------- unable to locate module {}\n\n", node_module);
            }
        }

        Err(format!(
            "Unable to resolve `{}` referred by `{}`",
            location.display(),
            referrer_content.get_absolute_path(self)?.display()
        ).into())

    }

    /*
    pub async fn load_v1(ctx: &Arc<Context>) -> Result<Db> {

        let node_modules = &ctx.node_modules;

        let list = WalkDir::new(&node_modules)
            .into_iter()
            .flatten()
            .filter_map(|entry|{
                let path = entry.path();

                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name == "package.json" {
                    Some(path.to_path_buf())
                } else {
                    None
                }
            });

        // let mut node_modules = Vec::new();
        // let mut file_content = Vec::new();
        for absolute in list {
            // println!("`{}`", absolute.display());
            let text = std::fs::read_to_string(&absolute)?;
            let package_json: PackageJson = serde_json::from_str(&text)?;
            println!("processing node module `{}`",absolute.display());
            let node_module = Arc::new(NodeModule::load(&ctx,package_json)?);
            println!("... done ... processing node module `{}`",absolute.display());
            // println!("{} files", node_module.files.len());
            db.insert_node_module(&node_module)?;
            // file_content.extend(node_module.files.clone());
            // file_content.extend(
            //     node_module.exports
            //     .iter()
            //     .map(|(_k,v)|v.clone())
            //     .collect::<Vec<_>>()
            // );

            // node_modules.push(node_module);
        }

        // let mut node_modules_by_id = HashMap::new();
        // let mut node_modules_by_name = HashMap::new();
        // let mut node_modules_by_absolute = HashMap::new();
        // for node_module in node_modules.iter() {
        //     db.register_node_module(&node_module);
            // node_modules_by_id.insert(node_module.id, node_module.clone());
            // node_modules_by_name.insert(node_module.name.clone(), node_module.clone());
            // if let Some(full_name) = &node_module.full_name {
            //     node_modules_by_name.insert(full_name.clone(), node_module.clone());
            // }
            // node_modules_by_absolute.insert(node_module.absolute.clone(), node_module.clone());
        // }

        // let mut file_content_by_absolute = HashMap::new();
        // for file_content in file_content.iter() {
        //     db.insert_file(&file_content);
        //     // file_content_by_absolute.insert(file_module.absolute.clone(), file_module.clone());
        // }

        println!("loading project file");
        let text = std::fs::read_to_string(&ctx.project_folder.join("package.json"))?;
        let package_json: PackageJson = serde_json::from_str(&text)?;
        let node_module = Arc::new(NodeModule::project(&ctx,package_json)?);
        db.insert_node_module(&node_module)?;
        // let content_type : ContentType = ctx.manifest.settings.project_type.clone().into();
        // let project = Arc::new(Content::load(
        //     &ctx,
        //     content_type,
        //     None,
        //     &ctx.project_file
        // )?);
        // // file_content.push(project.clone());
        // // file_content_by_absolute.insert(ctx.project_file.clone(), project.clone());
        // db.insert_file(&project);

        // TODO - load based on dependency declarations...
        // let dependencies = Dependencies::new();
        // if let Some(dependency_manifest) = &ctx.manifest.manifest {

        //     // let external_node_modules = HashMap<u64,
        //     for (kind, entries) in dependency_manifest.iter() {
        //         if kind == "node_modules" {

        //         }
        //         for (str_hex_id, info) in entries.iter() {

        //             let external_node_module: ExternalNodeModuleReference = info.try_into();

        //             let id = info.get("id");

        //             let id = hex_str_to_u64(str_hex_id)?;

        //             match kind.as_str() {
        //                 "node_modules" => {

        //                     continue;
        //                 },
        //                 _ => { }
        //             }
        //         }
        //     }

        //     for (kind, entries) in dependency_manifest.iter() {
        //         for (str_hex_id, reference) in entries.iter() {
        //             let id = hex_str_to_u64(str_hex_id)?;
        //             let content = match kind.as_str() {
        //                 "modules" => {
        //                     Arc::new(Content::external(ctx, ContentType::Module, id, reference)?)
        //                 },
        //                 "scripts" => {
        //                     Arc::new(Content::external(ctx, ContentType::Script, id, reference)?)
        //                 },
        //                 "styles" => {
        //                     Arc::new(Content::external(ctx, ContentType::Style, id, reference)?)
        //                 },
        //                 _ => { }
        //             };
        //             file_content.push(content.clone());
        //             file_content_by_absolute.insert(content.absolute.clone(), content.clone());
        //         }
        //     }
        // }


        Ok(db)

    }
    */

    // pub fn load_dependencies()

    pub fn locate_file(&self, location: &str) -> Result<Option<FileContext>> {
        let absolute = self.ctx.node_modules.join(location);
        if absolute.canonicalize().is_ok() {
            let node_module_name = NodeModule::get_name_from_location(location);

            let node_module = self.node_modules_by_name.get(&node_module_name);
            if let Some(node_module) = node_module {
                let file_context = FileContext::new(node_module.base_folder().clone(), absolute);
                Ok(Some(file_context))
            } else {
                Ok(None)
            }
        } else {
            log_error!("Unable to locate `{}` in node_modules", location);
            Ok(None)
        }

        // let absolute = self.ctx.node_modules.join(location);
        // let folder = absolute.parent().unwrap();
        // let package_json = search_upwards(folder, "package.json").unwrap();
    }

    pub fn locate(
        &self,
        // ctx: &Context,
        location: &str,
        referrer: &Arc<Content>,
        // node_module : u64,
        // referrer: P
    ) -> Result<Option<Arc<Content>>> {
        // check if location matches any node modules
        if let Some(node_module) = self.node_modules_by_name.get(location) {
            if let Some(default) = &node_module.default {
                return Ok(Some(default.clone()));
            }
        }

        let base_location = self
            // .node_modules_by_id
            .get_node_module(&referrer.node_module_id)
            .expect(&format!("Unable to locate node module for `{}`", location))
            .base_folder()
            .join(location);

        if let Some(file) = self.file_content_by_location.get(&base_location) {
            return Ok(Some(file.clone()));
        }

        // reference.referrer.parent().unwrap().join(location),

        Ok(None)
    }
}
