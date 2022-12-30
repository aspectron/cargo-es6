use crate::prelude::*;

pub struct Db {
    pub ctx : Arc<Context>,
    pub node_modules : Vec<Arc<NodeModule>>,
    pub node_modules_by_name : HashMap<String, Arc<NodeModule>>,
    pub node_modules_by_absolute : HashMap<PathBuf, Arc<NodeModule>>,
    pub file_content : Vec<Arc<Content>>,
    pub file_content_by_absolute : HashMap<PathBuf, Arc<Content>>,
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

        let node_modules = Db {
            ctx : ctx.clone(),
            node_modules,
            node_modules_by_name,
            node_modules_by_absolute,
            file_content,
            file_content_by_absolute,
        };

        Ok(node_modules)

    }

    // pub async fn resolve(&mut self, location: &str, referrer : &Path) -> Result<Option<Arc<Content>>> {
    // pub fn resolve(&mut self, reference : &Reference, content : &Content) -> Result<Option<Arc<Content>>> {

    //     let location = &reference.location.clone();
    //     let referrer = content.absolute.clone();

    //     // let location = &self.ctx.replace(&location);
        
    //     let relative_re = Regex::new(r"^.?.?/").unwrap();

    //     let absolute = if relative_re.is_match(location) {
    //         let location = if location.starts_with("./") {
    //             &location[2..]
    //         } else if location.starts_with("/") {
    //             &location[1..]
    //         } else {
    //             location
    //         };
            
    //         println!("--- relative: `{}`",location);
    //         let paths = [
    //             reference.referrer.parent().unwrap().join(location),
    //             content.absolute.parent().unwrap().join(location),
    //             content.folder.join(location),
    //             content.base_folder.join(location)
    //         ];

    //         let path = paths
    //             .iter()
    //             .find(|p|{
    //                 p.canonicalize().is_ok()
    //             });

    //         if path.is_none() {
    //             return Err(format!("Unable to resolve location `{}`", location).into())
    //         }

    //         Path::new(&path.unwrap().canonicalize()?).to_path_buf()
    //     } else {
    //         println!("---- absolute");
    //         if let Some(node_module) = self.node_modules_by_name.get(location) {
    //             if let Some(module) = &node_module.root {
    //                 return Ok(Some(module.clone()));
    //             }
    //         }

    //         let absolute = self.ctx.node_modules.join(location);
    //         absolute
    //     };

    //     let absolute = {
    //         if absolute.is_file() {
    //             absolute
    //         } else {
    //             let absolute = content.base_folder.join(location);
    //             if absolute.is_file() {
    //                 absolute
    //             } else {
    //                 let relative = referrer.strip_prefix(&self.ctx.project_folder)?;
    //                 let target = absolute.strip_prefix(&self.ctx.project_folder)?;
    
    //                 // if !self.ctx.ignore.is_match(&relative.to_string_lossy()) && !self.ctx.ignore.is_match(&target.to_string_lossy()) {
    //                     log_warn!("Resolver","+--");
    //                     log_warn!("","| Unable to resolve: `{}`", style(location).yellow());
    //                     log_warn!("","| absolute: `{}`", style(absolute.display()).yellow());
    //                     log_warn!("","| referrer: `{}`", style(referrer.display()).yellow());
    //                     log_warn!("","+--");
    //                 // }
    //                 return Ok(None);
    //             }
    //         }
    //     };

    //     if let Some(target) = self.file_content_by_absolute.get(&absolute) {
    //         Ok(Some(target.clone()))
    //     } else {
    //         let content_type = match reference.kind {
    //             ReferenceKind::Module | ReferenceKind::Export => {
    //                 ContentType::Module
    //             },
    //             ReferenceKind::Script => {
    //                 ContentType::Script
    //             },
    //             ReferenceKind::Style => {
    //                 ContentType::Style
    //             }
    //         };

    //         println!("creating ref content for: `{}`",reference.location);
    //         let base_folder = content.base_folder.clone();
    //         let ref_content = Arc::new(Content::load(&self.ctx, content_type, base_folder, absolute.clone())?);
    //         self.file_content.push(ref_content.clone());
    //         self.file_content_by_absolute.insert(absolute.clone(), ref_content.clone());

    //         if let Some(references) = &ref_content.references {
    //             for ref_reference in references.iter() {
    //                 println!("resolving location: {}", ref_reference.location);
    //                 println!("resolving referrer: {}", ref_reference.referrer.display());
    //                 self.resolve(ref_reference, &ref_content)?;
    //             }
    //         }

    //         Ok(Some(ref_content))
    //     }

    //     // if let Some(target) = self.file_content_by_absolute.get(&absolute) {
    //     //     Ok(Some(target.clone()))
    //     // } else {

    //     //     // last resort, try base folder
    //     //     let absolute = content.base_folder.join(location);
    //     //     if let Some(target) = self.file_content_by_absolute.get(&absolute) {
    //     //         Ok(Some(target.clone()))
    //     //     } else {
    
    //     //     }
    //     // }

    // }
}

