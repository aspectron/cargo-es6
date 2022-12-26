use crate::prelude::*;

pub struct Modules {
    ctx : Arc<Context>,
    pub node_modules : Vec<Arc<NodeModule>>,
    pub node_modules_by_name : HashMap<String, Arc<NodeModule>>,
    pub node_modules_by_absolute : HashMap<PathBuf, Arc<NodeModule>>,
    pub file_modules : Vec<Arc<FileModule>>,
    pub file_modules_by_absolute : HashMap<PathBuf, Arc<FileModule>>,
}

impl Modules {
// pub async fn load_node_modules(ctx: &Context) -> Result<HashMap<String,Arc<NodeModule>>> {
    pub async fn load(ctx: &Arc<Context>) -> Result<Modules> {

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
        let mut file_modules = Vec::new();
        for absolute in list {
            // println!("`{}`", absolute.display());
            let text = std::fs::read_to_string(&absolute)?;
            let package_json: PackageJson = serde_json::from_str(&text)?;
            let node_module = Arc::new(NodeModule::load(&absolute, package_json).await?);
            // println!("{} files", node_module.files.len());
            
            file_modules.extend(node_module.files.clone());
            file_modules.extend(
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

        let mut file_modules_by_absolute = HashMap::new();
        for file_module in file_modules.iter() {
            file_modules_by_absolute.insert(file_module.absolute.clone(), file_module.clone());
        }

        let project = Arc::new(FileModule::load(&ctx.project_folder, &ctx.project_file).await?);
        file_modules.push(project.clone());
        file_modules_by_absolute.insert(ctx.project_file.clone(), project.clone());

        let node_modules = Modules {
            ctx : ctx.clone(),
            node_modules,
            node_modules_by_name,
            node_modules_by_absolute,
            file_modules,
            file_modules_by_absolute,
        };

        Ok(node_modules)

    }

    pub async fn resolve(&self, location: &str, referrer : &Path) -> Result<Option<Arc<FileModule>>> {

        let relative_re = Regex::new(r"^.?.?/").unwrap();
        let absolute = if relative_re.is_match(location) {
            
            let location = if location.starts_with("./") {
                &location[2..]
            } else if location.starts_with("/") {
                &location[1..]
            } else {
                location
            };

            let origin = referrer.parent().unwrap();
            // println!("origin: {}", origin.display());
            let absolute = origin.join(location).canonicalize().await?;
            absolute
        } else {

            if let Some(node_module) = self.node_modules_by_name.get(location) {
                if let Some(module) = &node_module.root {
                    return Ok(Some(module.clone()));
                }
            }

            let absolute = self.ctx.node_modules.join(location);
            absolute
        };


        if let Some(target) = self.file_modules_by_absolute.get(&absolute) {
            Ok(Some(target.clone()))
        } else {
            log_warn!("Resolver","+--");
            log_warn!("","| Unable to resolve: `{}`", style(location).yellow());
            log_warn!("","| absolute: `{}`", style(absolute.display()).yellow());
            log_warn!("","| referrer: `{}`", style(referrer.display()).yellow());
            log_warn!("","+--");
            Ok(None)
        }

    }
}

