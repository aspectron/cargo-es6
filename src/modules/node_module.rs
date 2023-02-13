use crate::prelude::*;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Export {
//     pub default : String,
//     pub development : Option<String>,
// }

pub type Exports = HashMap<String, serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageJson {
    pub version: Option<String>,
    pub name: Option<String>,
    // pub exports : Option<HashMap<String,HashMap<String,String>>>,
    pub exports: Option<HashMap<String, serde_json::Value>>,
    pub dependencies: Option<HashMap<String, String>>,
    pub files: Option<Vec<String>>,
    pub module: Option<String>,
    pub main: Option<String>,
}

#[derive(Debug)]
pub struct NodeModule {
    pub id: u64,
    // if false, represents the project
    pub is_project: bool,
    // pub default : Option<u64>,
    // pub folder : PathBuf,
    // pub absolute : PathBuf,
    pub version: Option<String>,
    pub name: String,
    pub alt_name: String,
    pub default: Option<Arc<Content>>,
    pub exports: HashMap<String, HashMap<String, Arc<Content>>>,
    pub files: Vec<Arc<Content>>,
    pub package_json: PackageJson,
    // pub module_folder: PathBuf,
    pub base_folder: PathBuf,
}

impl NodeModule {
    // pub fn project(db: &mut Db, package_json: PackageJson) -> Result<NodeModule> {

    //     let name = if let Some(name) = &package_json.name {

    //     } else {

    //     };

    //     let name = package_json.name.as_ref().unwrap_or(&db.ctx.manifest.settings.name);
    //     let (name, alt_name) = Self::get_alt_name(&name);
    //     let id = name.u64_hash();
    //     // let base_folder = ctx.project_folder;//.join(name);

    //     let node_module = NodeModule {
    //         id,
    //         is_project : true,
    //         default : None,
    //         version : package_json.version.clone(),
    //         name,
    //         alt_name,
    //         exports : HashMap::new(),
    //         files : Vec::new(),
    //     };

    //     Ok(node_module)
    // }

    // pub fn load<P>(ctx: &Context, absolute: P, package_json: PackageJson) -> Result<NodeModule>
    // pub fn load(db: &mut Db, package_json: PackageJson) -> Result<NodeModule> {
    // pub fn load(db: &mut Db, module_name: &str) -> Result<NodeModule> {
    pub fn load(db: &mut Db, module_folder: &PathBuf) -> Result<NodeModule> {
        // println!("loading {}", module_folder.display());

        let text = std::fs::read_to_string(&module_folder.join("package.json"))?;
        let package_json: PackageJson = serde_json::from_str(&text)?;

        // ctx.project_folder.join("package.json")

        // let absolute = absolute.as_ref();
        // let folder = absolute.parent().unwrap();

        let (name, alt_name) = Self::get_alt_name(package_json.name.as_ref().expect(&format!(
            "`package.json` missing name property: {:#?}",
            package_json
        )));
        // let base_folder = ctx.node_modules.join(name);
        let id = name.u64_hash();

        // if let Some(node_module) = db.get_node_module(&id) {
        //     return Ok(node_module.clone())
        // }
        // let mut hasher = DefaultHasher::new();
        // // let n = full_name.as_ref().unwrap_or(&name);
        // name.hash(&mut hasher);
        // let id = hasher.finish();

        let base_folder = module_folder
            .clone()
            .strip_prefix(&db.ctx.project_folder)?
            .to_path_buf();

        // let mut files = Vec::new();
        // for relative in relative_files.iter() {
        //     files.push(Arc::new(Content::load(ctx,ContentType::Module,folder,relative).await?));
        // }

        let mut node_module = NodeModule {
            id,
            is_project: false,
            default: None,
            version: package_json.version.clone(),
            name: name.clone(),
            alt_name,
            exports: HashMap::new(),
            files: Vec::new(),
            package_json,
            base_folder,
            // module_folder : module_folder.clone(),
        };

        // if let Some(exports) = package_json.exports {
        //     node_module.digest_exports(&db.ctx, &exports)?;
        // }

        // if let Some(include_all_exports) = &db.ctx.manifest.settings.include_all_exports {
        //     if include_all_exports.contains(&node_module.name) || include_all_exports.contains(&node_module.alt_name) {
        //         node_module.digest_files(&db.ctx, package_json.files)?;
        //     }
        // }

        // let mut main_file = node_module.package_json
        //     .module
        //     .clone()
        //     .or(node_module.package_json.main.clone())
        //     .unwrap_or("index.js".to_string());
        // if main_file.len() == 0 {
        //     main_file = "index.js".to_string();
        // }

        // let base_folder = node_module.base_folder()?;
        // println!("processing default");
        // let default = match module_folder.join(Path::new(&main_file)).canonicalize() {
        //     Ok(absolute) => {
        //         let location = absolute.strip_prefix(&module_folder).expect("default - strip prefix").to_path_buf();
        //         // let m = Arc::new(Content::load(ctx,ContentType::Module, folder,&relative)?);
        //         println!("node module ... loading location `{}`",location.display());
        //         let m = Arc::new(Content::load(
        //             db,
        //             ContentType::Module,
        //             id,
        //             module_folder,
        //             &location
        //         )?);
        //         files.push(m.clone());
        //         Some(m)
        //     },
        //     Err(_) => {
        //         log_warn!("Warning","node module `{}` contains no root file",name);
        //         None
        //     }
        // };
        // node_module.default = default;
        // println!("... processing default done ...");

        // Ok(Arc::new(node_module))

        // let node_module = Arc::new(node_module);

        // db.insert_node_module(&node_module);

        Ok(node_module)
    }

    pub fn main_file(&self, ctx: &Context) -> Option<PathBuf> {
        let mut main_file = self
            .package_json
            .module
            .clone()
            .or(self.package_json.main.clone())
            .unwrap_or("index.js".to_string());
        if main_file.len() == 0 {
            main_file = "index.js".to_string();
        }

        let main_file_absolute = ctx.project_folder.join(&self.base_folder).join(&main_file);
        if main_file_absolute.exists() {
            Some(PathBuf::from(main_file))
        } else {
            None
        }
    }

    pub fn base_folder(&self) -> &PathBuf {
        &self.base_folder
    }

    pub fn get_absolute_path(&self, db: &Db) -> PathBuf {
        db.ctx.project_folder.join(&self.base_folder)
    }

    pub fn dependencies(&self) -> Option<Vec<&String>> {
        if let Some(dependencies) = &self.package_json.dependencies {
            let dependencies = dependencies.keys().collect::<Vec<_>>();
            if dependencies.is_empty() {
                None
            } else {
                Some(dependencies)
            }
        } else {
            None
        }
    }

    // pub fn resolve(&self, db : &mut Db) -> Result<()> {
    //     if let Some(dependencies) = &self.package_json.dependencies {
    //         for (dependency, _) in dependencies.iter() {
    //             db.resolve_node_module_dependency(self, dependency)?;
    //         }
    //     }

    //     Ok(())
    // }

    // pub fn base_folder(&self) -> Result<PathBuf> {
    // // pub fn absolute_folder(&self) -> Result<PathBuf> {
    //     let path = if self.is_project {
    //         Path::new("").to_path_buf()

    //     } else {
    //         Path::new("node_modules").join(&self.name)
    //         // ctx.node_modules.join("node_modules").join(self.name).strip_prefix(ctx.project_folder)?.to_path_buf()
    //     };

    //     Ok(path)
    // }

    // pub fn absolute_folder(&self, ctx : &Context) -> Result<PathBuf> {
    // // pub fn absolute_folder(&self) -> Result<PathBuf> {
    //     let path = if self.is_project {
    //         // Path::new("").to_path_buf()
    //         ctx.project_folder.clone()
    //     } else {
    //         ctx.node_modules.join(&self.name)
    //         // Path::new("node_modules").join(self.name)
    //         // ctx.node_modules.join("node_modules").join(self.name).strip_prefix(ctx.project_folder)?.to_path_buf()
    //     };

    //     Ok(path)
    // }

    // pub fn absolute_folder(&self, ctx : &Context) -> PathBuf {
    //     ctx.node_modules.join(self.name)
    // }
    /*
        pub fn digest_exports(&mut self, ctx: &Context, node_module_exports : &Exports) -> Result<()> {

            let base_folder = self.base_folder()?;
            for (target, exports) in node_module_exports.iter() {
                println!("... loading export ...");
                println!("... loading export file: `{}`", target);
                // println!("... loading export export: `{}`", _export.default);

                let export_base_location = base_folder.join(&target);
                let export_base_location = export_base_location.normalize()?;
                let export_base_location = export_base_location.to_str().unwrap();

                let object = exports.as_object();
                if object.is_none() {
                    continue;
                }

                if let Some(object) = exports.as_object() {


                    for (name, file) in object.iter() {
                        let file = if let Some(file) = file.as_str() {
                            file.to_string()
                        } else {
                            log_warn!("Exports","invalid exoprt `{}` in `{}`", name, self.name);
                            continue;
                        };
                    // let location = Path::new(export.default);
                        let content = Content::load(
                            ctx,
                            ContentType::Module,
                            self.id,
                            &base_folder,
                            // &location.to_path_buf(),
                            // &Path::new(&export.default).to_path_buf()
                            &Path::new(&file).to_path_buf()
                        )?;
                        // let export_base_location = base_folder.join(&export.default);

                        let group = match self.exports.get_mut(name) {
                            Some(group) => group,
                            None => {
                                let group = HashMap::new();
                                self.exports.insert(name.to_string(), group);
                                self.exports.get_mut(name).unwrap()
                            }
                        };
                        group.insert(export_base_location.to_string(), Arc::new(content));
                    }
                }
                // if let Some(development) = &export.development {
                //     // let path = folder.join(development).canonicalize()?;
                //     let module = Content::load(
                //         ctx,
                //         ContentType::Module,
                //         self.id,
                //         base_folder,
                //         Path::new(&development).to_path_buf()
                //     )?;
                //     self.exports.insert(file.to_string(), Arc::new(module));
                // }
            }

            Ok(())
        }
    */

    /*
        pub fn digest_files(&mut self, ctx: &Context, files : Option<Vec<String>>) -> Result<()> {

            println!("digesting all files...");
            let absolute_folder = self.absolute_folder(&ctx)?;
            let filter = Filter::new(&["*.{js,mjs,css}"]);
            let relative_files = if let Some(files) = files {

                let globs = files
                    .iter()
                    .map(|s|s.as_str())
                    .collect::<Vec<_>>();
                let aggregator = Filter::new(&globs);
                let list = get_files(&absolute_folder, Some(&aggregator),Some(&filter))?;
                list
            } else {
                let aggregator = Filter::new(&["*.{js,mjs,css}"]);
                get_files(&absolute_folder, Some(&aggregator),None)?
            };

            for location in relative_files {

                println!("... -> location: `{}`", location.display());

                self.files.push(Arc::new(Content::load(
                    ctx,
                    ContentType::Module,
                    self.id,
                    &absolute_folder,
                    &location
                )?));
            }

            Ok(())
        }
    */
    // pub fn full_name(&self) -> String {
    //     self.full_name.as_ref().unwrap_or(&self.name).to_string()
    // }

    pub fn get_alt_name(full_name: &str) -> (String, String) {
        if full_name.starts_with("@") {
            let alt_name = full_name
                .split("/")
                .collect::<Vec<_>>()
                .get(1)
                .unwrap()
                .to_string();
            (full_name.to_string(), alt_name)
        } else {
            (full_name.to_string(), full_name.to_string())
        }
    }

    // pub fn folder(&self, ctx: &Context) -> PathBuf {
    //     ctx.node_modules.join(&self.name)
    // }

    pub fn get_name_from_location(location: &str) -> String {
        if location.starts_with("@") {
            let components = Path::new(location)
                .components()
                .into_iter()
                .map(|c| c.as_os_str().to_str().unwrap())
                .collect::<Vec<_>>();
            let mut node_module = Vec::new();
            for n in 0..std::cmp::min(2, components.len()) {
                node_module.push(components[n].to_string());
            }
            node_module.join("/")
        } else {
            let components = Path::new(location)
                .components()
                .into_iter()
                .map(|c| c.as_os_str().to_str().unwrap())
                .collect::<Vec<_>>();
            components[0].to_string()
        }
    }
}

// impl TryFrom<(&Db,u64,HashMap<String,toml::Value>)> for NodeModule {
//     type Error = Error;
//     fn try_from((db,id, map): (&Db,u64,HashMap<String, toml::Value>)) -> Result<NodeModule> {
//         let name = map
//             .get("name")
//             .and_then(toml::Value::as_str)
//             .map(|s|s.to_string())
//             .expect("missing node_module name");
//         let version = map
//             .get("version")
//             .and_then(toml::Value::as_str)
//             .map(|v|v.to_string());
//         let default = map
//             .get("default")
//             .and_then(toml::Value::as_str)
//             .and_then(|v|hex_str_to_u64(v).ok());
//             // .map(|v|hex_str_to_u64(v)?);

//         let default = default.map(|id|{
//             db.file_content_by_id
//                 .get(&id)
//                 .cloned()
//                 .expect(&format!("unable to resolve default export id 0x{:16x}",id))
//         });

//         let (name, alt_name) = Self::get_alt_name(&name);
//         let node_module = NodeModule {
//             id,
//             is_project : false,
//             default,// : //root.as_ref().map(|m|m.id),
//             // folder : folder.to_path_buf(),
//             // absolute : absolute.to_path_buf(),
//             version,// : package_json.version.clone(),
//             name,// : package_json.name.clone(),
//             alt_name,// : package_json.name.clone(),
//             exports : HashMap::new(),
//             files : Vec::new(),
//             // root,
//         };

//         Ok(node_module)

//         // Ok(ExternalNodeModuleReference {
//         //     id,
//         //     name,
//         //     full_name,
//         //     version,
//         // })
//     }
// }

impl ToString for NodeModule {
    fn to_string(&self) -> String {
        let fields = [
            // ("name", Some(self.name.clone())),
            ("name", Some(self.name.clone())),
            ("version", self.version.clone()),
            (
                "default",
                self.default
                    .as_ref()
                    .map(|content| u64_to_hex_str(&content.id)),
            ),
        ]
        .iter()
        .filter(|(_k, v)| v.is_some())
        .map(|(k, v)| format!("{} = \"{}\"", k, v.as_ref().unwrap()))
        .collect::<Vec<_>>()
        .join(", ");
        format!("{{ {} }}", fields)
    }
}
