use crate::prelude::*;

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
    pub node_modules : Vec<Arc<NodeModule>>,
    pub node_modules_by_name : HashMap<String, Arc<NodeModule>>,
    pub node_modules_by_absolute : HashMap<PathBuf, Arc<NodeModule>>,
    pub file_content : Vec<Arc<Content>>,
    pub file_content_by_absolute : HashMap<PathBuf, Arc<Content>>,
    pub dependencies : Dependencies,
}

impl Db {
    pub async fn load(ctx: &Arc<Context>) -> Result<Db> {

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

        let mut node_modules = Vec::new();
        let mut file_content = Vec::new();
        for absolute in list {
            // println!("`{}`", absolute.display());
            let text = std::fs::read_to_string(&absolute)?;
            let package_json: PackageJson = serde_json::from_str(&text)?;
            let node_module = Arc::new(NodeModule::load(&ctx,&absolute, package_json)?);
            // println!("{} files", node_module.files.len());
            
            file_content.extend(node_module.files.clone());
            file_content.extend(
                node_module.exports
                .iter()
                .map(|(_k,v)|v.clone())
                .collect::<Vec<_>>()
            );

            node_modules.push(node_module);
        }

        let mut node_modules_by_name = HashMap::new();
        let mut node_modules_by_absolute = HashMap::new();
        for node_module in node_modules.iter() {
            node_modules_by_name.insert(node_module.name.clone(), node_module.clone());
            if let Some(full_name) = &node_module.full_name {
                node_modules_by_name.insert(full_name.clone(), node_module.clone());
            }
            node_modules_by_absolute.insert(node_module.absolute.clone(), node_module.clone());
        }

        let mut file_content_by_absolute = HashMap::new();
        for file_module in file_content.iter() {
            file_content_by_absolute.insert(file_module.absolute.clone(), file_module.clone());
        }

        let content_type : ContentType = ctx.manifest.settings.project_type.clone().into();
        let project = Arc::new(Content::load(&ctx,content_type,&ctx.project_folder, &ctx.project_file)?);
        file_content.push(project.clone());
        file_content_by_absolute.insert(ctx.project_file.clone(), project.clone());

        // TODO - load based on dependency declarations...
        let dependencies = Dependencies::new();
        for (kind, entries) in ctx.manifest.manifest.iter() {
            for (str_hex_id, path) in entries.iter() {
                let _id = id_from_string(str_hex_id)?;
                let _path = Path::new(path).to_path_buf();
                // println!("> {} {} {}", kind, id, path.display());
                match kind.as_str() {
                    "modules" => {
                        let _content_type = ContentType::Module;
                        // TODO
                    },
                    "scripts" => {
                        let _content_type = ContentType::Script;
                        // TODO
                    },
                    "styles" => { 
                        let _content_type = ContentType::Style;
                        // TODO
                    },
                    "node_modules" => {
                        // TODO
                    },
                    _ => {
                        panic!("Unknown dependency content type section: {}", kind);
                    }
                }
            }

        }

        let node_modules = Db {
            ctx : ctx.clone(),
            node_modules,
            node_modules_by_name,
            node_modules_by_absolute,
            file_content,
            file_content_by_absolute,
            dependencies
        };

        Ok(node_modules)

    }

    // pub fn load_dependencies()

}

