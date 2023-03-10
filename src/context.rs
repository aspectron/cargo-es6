use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Options {}

// impl Default for Options {
//     fn default() -> Self {
//         Options {}
//     }
// }

#[derive(Debug)]
pub enum Replace {
    Location {
        regex: Option<Regex>,
        from: Option<String>,
        to: String,
    },
}

#[derive(Debug)]
pub struct Context {
    pub manifest: Manifest,
    pub manifest_toml: PathBuf,
    pub manifest_folder: PathBuf,
    pub target_folder: PathBuf,
    pub project_folder: PathBuf,
    pub project_node_module: Option<String>,
    pub node_modules: PathBuf,
    pub package_json: PathBuf,
    pub ignore: Filter,
    pub replace: Option<Vec<Replace>>,
}

impl Context {
    pub fn create(location: Option<String>, _options: Options) -> Result<Context> {
        let manifest_toml = Manifest::locate(location)?;
        log_info!("Manifest", "`{}`", manifest_toml.to_str().unwrap());
        let manifest = Manifest::load(&manifest_toml)?;
        let manifest_folder = manifest_toml.parent().unwrap().to_path_buf();

        let project_folder = manifest_folder
            .join(&manifest.settings.project)
            .canonicalize()?;

        let project_node_module = manifest.settings.module.clone(); //.expect("Manifest is missing [settings.module] option");
        let node_modules = project_folder.join("node_modules");
        let package_json = project_folder.join("package.json");
        let target_folder = manifest_folder.join(&manifest.settings.target);

        log_info!("Project", "`{}`", project_folder.to_str().unwrap());
        log_info!("Target", "`{}`", target_folder.to_str().unwrap());

        let ignore = if let Some(ignore) = &manifest.settings.ignore {
            Filter::new(&ignore.iter().map(|s| s.as_str()).collect::<Vec<_>>())
        } else {
            Filter::default()
        };

        let mut replace = Vec::new();
        if let Some(items) = &manifest.settings.replace {
            for item in items {
                match item {
                    crate::manifest::Replace::Location { regex, from, to } => {
                        if regex.is_none() && from.is_none() {
                            return Err("[settings].replace must contain `regex` or `from`".into());
                        }

                        let r = if let Some(regex) = regex {
                            let r = Regex::new(regex).map_err(|err| {
                                println!();
                                println!("Error compiling replace Regex expression: `{regex}`");
                                println!();
                                err
                            })?;
                            Some(r)
                        } else {
                            None
                        };

                        replace.push(Replace::Location {
                            regex: r,
                            from: from.clone(),
                            to: to.clone(),
                        });
                    }
                }
            }
        }

        let replace = if replace.is_empty() {
            None
        } else {
            Some(replace)
        };

        let ctx = Context {
            manifest,
            manifest_toml,
            manifest_folder,
            target_folder,
            project_folder,
            project_node_module,
            node_modules,
            package_json,
            ignore,
            replace,
        };

        Ok(ctx)
    }

    pub async fn ensure_folders(&self) -> Result<()> {
        let folders = [&self.target_folder];
        for folder in folders {
            if !std::path::Path::new(folder).exists() {
                std::fs::create_dir_all(folder)?;
            }
        }

        Ok(())
    }

    pub async fn clean(&self) -> Result<()> {
        // if self.target_folder_src.exists().await {
        //     // log_info!("Cleaning","`{}`",self.target_folder.display());
        //     async_std::fs::remove_dir_all(&self.target_folder_src).await?;
        // }
        Ok(())
    }

    pub fn replace(&self, text: &str) -> String {
        let mut text = text.to_string();
        if let Some(replace) = &self.replace {
            for r in replace.iter() {
                match r {
                    crate::context::Replace::Location { regex, from, to } => {
                        if let Some(regex) = regex {
                            text = regex.replace_all(&text, to).to_string();
                        } else if let Some(from) = from {
                            // TODO: CLEANUP
                            // if text.starts_with("$") {
                            //     println!("replacing {}",l);
                            //     l = location.replace(from,to);
                            //     println!("result {}",l);
                            // } else {
                            text = text.replace(from, to);
                            // }
                        } else {
                            panic!("replace directivec has no regex or from values");
                        }
                    }
                }
            }
        }

        text
    }
}
