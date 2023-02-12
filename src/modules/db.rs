use crate::prelude::*;

pub struct FileContext {
    pub base_folder : PathBuf,
    pub absolute : PathBuf,
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
    pub file_content : HashMap<String,Arc<Content>>,
    pub node_modules : HashMap<String,Arc<Content>>,
}
impl Dependencies {
    pub fn new() -> Dependencies {
        Dependencies {
            file_content : HashMap::new(),
            node_modules : HashMap::new(),
        }
    }
}

pub struct Db {
    pub ctx : Arc<Context>,
    // pub project_folder : PathBuf,
    pub node_modules : Vec<Arc<NodeModule>>,
    pub node_modules_by_id : HashMap<u64, Arc<NodeModule>>,
    pub node_modules_by_name : HashMap<String, Arc<NodeModule>>,
    // pub node_modules_by_absolute : HashMap<PathBuf, Arc<NodeModule>>,
    pub file_content : Vec<Arc<Content>>,
    pub file_content_by_id : HashMap<u64, Arc<Content>>,
    pub file_content_by_location : HashMap<PathBuf, Arc<Content>>,
    // pub dependencies : Dependencies,
}

impl Db {

    pub fn get_file(&self, id: &u64) -> Option<&Arc<Content>> {
        self.file_content_by_id.get(id)
    }

    pub fn get_node_module(&self, id: &u64) -> Option<&Arc<NodeModule>> {
        self.node_modules_by_id
            .get(id)
    }


    pub fn insert_file(&mut self, content: &Arc<Content>) -> Result<()>{
        self.file_content.push(content.clone());
        self.file_content_by_id.insert(content.id, content.clone());
        let location = self
            .get_node_module(&content.node_module_id)
            .expect(&format!("Unable to locate node module for `{}`", content.location.display()))
            .base_folder()?
            .join(&content.location);
        // self.file_content_by_location.insert(content.location.clone(), content.clone());
        self.file_content_by_location.insert(location, content.clone());
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
            ctx : ctx.clone(),
            node_modules : Vec::new(),
            node_modules_by_id : HashMap::new(),
            node_modules_by_name : HashMap::new(),
            // node_modules_by_absolute : HashMap::new(),
            file_content : Vec::new(),
            file_content_by_id : HashMap::new(),
            file_content_by_location : HashMap::new(),
            // dependencies
        }
    }

    pub fn insert_node_module(&mut self, node_module: &Arc<NodeModule>) -> Result<()> {

        for file in node_module.files.iter() {
            self.insert_file(&file)?;
        }

        // TODO
        for (target,exports) in node_module.exports.iter() {
            for (name, content) in exports.iter() {
                self.insert_file(content)?;
            }
        }

        self.node_modules.push(node_module.clone());
        self.node_modules_by_id.insert(node_module.id, node_module.clone());
        if node_module.name == node_module.alt_name {
            self.node_modules_by_name.insert(node_module.name.clone(),node_module.clone());
        } else {
            self.node_modules_by_name.insert(node_module.name.clone(),node_module.clone());
            self.node_modules_by_name.insert(node_module.alt_name.clone(),node_module.clone());
        }

        Ok(())
    }

    pub async fn load(ctx: &Arc<Context>) -> Result<Db> {

        let mut db = Db::new(ctx);

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

    // pub fn load_dependencies()

    pub fn locate_file(&self, location: &str) -> Result<Option<FileContext>> {



        let absolute = self.ctx.node_modules.join(location);
        if absolute.canonicalize().is_ok() {

            let node_module_name = NodeModule::get_name_from_location(location);

            let node_module = self.node_modules_by_name.get(&node_module_name);
            if let Some(node_module) = node_module {
                let file_context = FileContext::new(node_module.base_folder()?, absolute);
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
        location : &str,
        referrer : &Arc<Content>,
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
            .base_folder()?
            .join(location);

        if let Some(file) = self.file_content_by_location.get(&base_location) {
            return Ok(Some(file.clone()));
        }

        // reference.referrer.parent().unwrap().join(location),

        


        Ok(None)
    }
}

