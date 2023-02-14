use crate::prelude::*;

pub struct Builder {
    ctx: Arc<Context>,
}

impl Builder {
    pub fn new(ctx: Arc<Context>) -> Builder {
        Builder { ctx }
    }

    pub async fn execute(self: Arc<Builder>) -> Result<()> {
        if !self.ctx.package_json.exists() {
            return Err(format!(
                "`package.json` file not found at `{}`",
                self.ctx.package_json.display()
            )
            .into());
        }

        // if !self.ctx.node_modules.exists() {
        //     cmd!("npm", "install").dir(&self.ctx.project_folder).run()?;
        // }

        async_std::fs::create_dir_all(&self.ctx.target_folder)
            .await
            .unwrap();

        let db = Db::load(&self.ctx).await?;

        // println!("ready...");
        // for node_module in modules.node_modules.iter() {
        //     log_info!("Module","`{}` files: {} explicit exports: {}", style(&node_module.name).cyan(), node_module.files.len(), node_module.exports.len());
        // }

        // self.resolve(&mut modules).await?;

        // println!("");

        // let project_file_relative = self.ctx.project_file.strip_prefix(&self.ctx.project_folder).map_err(|_err|
        //     format!("Unable to locate project file `{}` in `{}`",self.ctx.project_file.display(),self.ctx.project_folder.display())
        // )?;

        let module = db.project_main.clone();

        // let module = db.file_content_by_location.get(project_file_relative);
        if let Some(module) = module {
            self.generate(&module, &db).await?;
        } else {
            return Err("Unabel to resolve project main module file".into());
        }

        self.build_wasm().await?;

        Ok(())
    }

    // pub async fn resolve(self: &Arc<Builder>, db: &mut Db) -> Result<()> {
    //     let file_content = db.file_content.clone();
    //     for content in file_content.iter() {
    //         content.resolve(db)?;
    //         // self.resolve_module(module,modules).await?;
    //     }

    //     Ok(())
    // }

    pub async fn gather_references(
        self: &Arc<Builder>,
        files: &[String],
        db: &Db,
    ) -> Result<Vec<Arc<Content>>> {
        let mut targets = vec![];

        for file in files.iter() {
            if let Some(content) = db.file_content_by_location.get(Path::new(file)) {
                if let Some(references) = content.references(db)? {
                    targets.extend(references);
                }
                // targets.push(content.clone());
            } else {
                return Err(format!("Unabel to resolve: `{file}`").into());
            }
        }

        Ok(targets)
    }

    // pub async fn get_references_v1(
    //     self: &Arc<Builder>,
    //     files: &Vec<String>,
    //     db: &Db,
    // ) -> Result<Vec<Arc<Content>>> {
    //     let mut targets = Vec::new();
    //     for file in files.iter() {
    //         let in_root = self.ctx.project_folder.join(file);
    //         if in_root.canonicalize().is_ok() {
    //             targets.push(in_root);
    //         } else {
    //             let in_node_modules = self.ctx.node_modules.join(file);
    //             if in_node_modules.canonicalize().is_ok() {
    //                 targets.push(in_node_modules);
    //             } else {
    //                 return Err(format!(
    //                     "get_references(): unable to locate `{file}` in project root or node modules"
    //                 )
    //                 .into());
    //             }
    //         }
    //     }

    //     let mut references = Vec::new();
    //     for target in targets.iter() {
    //         let module = db.file_content_by_location.get(target); //modules.resolve(location, referrer)
    //         if let Some(module) = module {
    //             if let Some(list) = module.references(db)? {
    //                 references.extend_from_slice(&list);
    //             }
    //             // let mut components = Vec::new();
    //             // for reference in references.iter() {
    //             //     // components.push(reference.clone());

    //             // }
    //             // references.iter().map(|reference| {}).collect::
    //         } else {
    //             return Err(format!(
    //                 "get_references(): unable to resolve module `{}`",
    //                 target.display()
    //             )
    //             .into());
    //         }
    //     }

    //     Ok(references)
    // }

    pub async fn generate(self: &Arc<Builder>, root_module: &Arc<Content>, db: &Db) -> Result<()> {
        // std::process::exit(1);
        let module_id_repr = "u64";

        let enums = if let Some(enums) = &self.ctx.manifest.settings.enums {
            let references = self.gather_references(&enums.exports, db).await?;
            let mut text = String::new();
            text.push_str(&format!(
                "\n#[allow(dead_code)]\n#[repr({module_id_repr})]\npub enum Modules {{\n"
                
            ));
            text.push_str("\tAll = 0,\n");
            for content in references.iter() {
                text.push_str(&format!("\t{} = {},\n", content.component, content.id()));
            }
            text.push_str("}\n");
            text
        } else {
            String::new()
        };

        // log_info!("Generating","`{}`",self.ctx.target_file.display());
        log_info!("Generating", "`{}`", self.ctx.target_folder.display());
        // let ident_kind = IdentKind::HexFull;
        let ident_kind = IdentKind::Full;

        let mut collection = Collection::new();
        root_module.gather(db, &mut collection)?;

        // println!("{:#?}", collection);
        // println!("TESTING ... ABORTING ...");
        // return Ok(());

        let mut content_rs = String::new();
        for content in collection.content.iter() {
            if !content.external {
                // println!("processing internal content {}", content.id());
                content_rs += &format!(
                    "pub const {} : &'static str = r###\"\n",
                    content.ident(&ident_kind)
                );
                content_rs += content
                    .contents
                    .as_ref()
                    .expect("missing content in internal instance");
                content_rs += "\n\"###;\n\n";
            }
        }

        let lib_rs = r###"
mod content;
mod context;
pub use context::*;
"###;

        let mut context_rs = String::new();
        context_rs += r###"
use std::sync::{
    Arc,
    Mutex,
    atomic::AtomicBool
};
use workflow_dom::loader::{
    ContentType,
    ContentList,
    Id,
    Reference,
    declare,
    Context as Inner,
};
pub use workflow_dom::loader::{
    Content
};
pub use workflow_dom::result::Result;
pub use workflow_dom::error::Error;
use super::content;
"###;
        context_rs += &format!("\nconst ROOT: Id = {};\n", root_module.id());

        context_rs += r###"
pub struct Context {
    inner : Arc<Inner>
}

impl Context {

    pub async fn load(&self, list : &[Id]) -> Result<()> {
        self.inner.load_ids(list).await?;
        Ok(())
    }

    pub async fn load_all(&self) -> Result<()> {
        self.inner.load_ids(&[ROOT]).await?;
        Ok(())
    }

    pub fn get(&self, id: &Id) -> Option<Arc<Content>> {
        self.inner.get(id)
    }
    
    pub fn url<'l>(&'l self, id: &Id) -> Option<String> {
        self.inner.get(id).map(|c|c.url()).unwrap_or(None)
    }
    
}
        "###;

        if !self.ctx.manifest.settings.verbose.unwrap_or(false) {
            context_rs = context_rs.replace("log_info", "// log_info");
        }

        context_rs += &enums;

        let mut table = Vec::new();
        for content in collection.content.iter() {
            let mut references = Vec::new();
            if let Some(targets) = &content.references {
                for reference in targets.iter() {
                    // match reference.kind {
                    let target = reference.content();
                    if let Some(target) = target {
                        // references.push(target);
                        match reference.kind {
                            ReferenceKind::Style => {
                                references.push(format!("(Reference::Style,None,{})", target.id()));
                            }
                            ReferenceKind::Module => {
                                if let Some(what) = &reference.what {
                                    references.push(format!(
                                        "(Reference::Module,Some(\"{}\"),{})",
                                        what,
                                        target.id()
                                    ));
                                } else {
                                    references
                                        .push(format!("(Reference::Module,None,{})", target.id()));
                                }
                            }
                            ReferenceKind::Script => {
                                references
                                    .push(format!("(Reference::Script,None,{})", target.id()));
                            }
                            ReferenceKind::Export => {
                                references.push(format!(
                                    "(Reference::Export,Some(\"{}\"),{})",
                                    reference.what.as_ref().unwrap(),
                                    target.id()
                                ));
                            }
                        }
                    } else {
                    }
                }
            }

            let references = if references.is_empty() {
                "None".to_string()
            } else {
                format!("Some(&[\n\t\t{}\n\t])", references.join(",\n\t\t"))
            };

            let definition = format!(
                "Arc::new(Content {{\n\
                \tcontent_type : {},\n\
                \turl : Mutex::new(None),\n\
                \tid : {},\n\
                \tident : \"{}\",\n\
                \tcontent : content::{},\n\
                \treferences : {},\n\
                \tis_loaded : AtomicBool::new(false),\n\
            }})",
                content.content_type.to_string(),
                content.id(),
                content.ident(&ident_kind).to_lowercase(),
                content.ident(&ident_kind),
                references,
            );

            table.push(format!("(0x{:x},{})", content.id, definition));
        }
        let table = table.join(",\n");
        context_rs += r###"
        impl Context {
            pub fn new() -> Self {

        "###;
        // context_rs += &format!("\nlet content : ContentList = [\n{}\n].into_iter().collect();\n", table);
        context_rs += &format!("\nlet content : ContentList = &[\n{table}\n];\n");
        context_rs += r###"
                Context {
                    inner : declare(content)
                }
            }
        }
        "###;

        let path_lib_rs = self.ctx.target_folder.join("mod.rs");
        let path_content_rs = self.ctx.target_folder.join("content.rs");
        let path_modules_rs = self.ctx.target_folder.join("context.rs");
        async_std::fs::write(&path_lib_rs, &lib_rs).await?;
        async_std::fs::write(&path_content_rs, &content_rs).await?;
        async_std::fs::write(&path_modules_rs, &context_rs).await?;

        self.update_manifest(db, &collection)?;

        let mut file_size = 0.0;
        // file_size += std::fs::metadata(&path_lib_rs)?.len() as f64 / 1024.0;
        file_size += std::fs::metadata(&path_content_rs)?.len() as f64 / 1024.0;
        // file_size += std::fs::metadata(&path_modules_rs)?.len() as f64 / 1024.0;
        log_info!(
            "Generating",
            "... modules: {} content file size: {:1.0} Kb",
            collection.content.len(),
            file_size
        );

        Ok(())
    }

    pub async fn build_wasm(&self) -> Result<()> {
        if let Some(wasm) = &self.ctx.manifest.settings.wasm {
            // wasm-pack build --dev --target web --out-name $NAME --out-dir root/wasm
            // wasm-pack build --target web --out-name $NAME --out-dir root/wasm

            let folder = self.ctx.project_folder.join(&wasm.folder).canonicalize()?;

            let outdir = folder.join(&wasm.outdir); //.join(wasm.name);
            let outdir = outdir.to_str().unwrap();

            let mut args = Vec::new();
            args.push("build");
            if wasm.dev.unwrap_or(false) {
                args.push("--dev");
            }
            args.extend_from_slice(&[
                "--target",
                "web",
                "--out-name",
                &wasm.name,
                "--out-dir",
                outdir,
            ]);

            cmd("wasm-pack", &args).dir(folder).run()?;
        }

        Ok(())
    }

    pub fn update_manifest(&self, db: &Db, _collection: &Collection) -> Result<()> {
        // manifest_toml += "[manifest]\n\n\n";
        // let mut manifest_modules = Vec::new();
        // let mut manifest_scripts = Vec::new();
        // let mut manifest_styles = Vec::new();

        // for content in collection.content.iter() {

        //     let reference: ExternalContentReference = (&self.ctx,content).try_into()?;

        //     let manifest_entry = format!("{} = {}",
        //         // content.content_type.to_manifest_type(),
        //         content.id(),
        //         reference.to_string()
        //         // content.absolute.strip_prefix(&self.ctx.project_folder)?.to_str().unwrap()
        //     );

        //     match content.content_type {
        //         ContentType::Module => {
        //             manifest_modules.push(manifest_entry);
        //         },
        //         ContentType::Script => {
        //             manifest_scripts.push(manifest_entry);
        //         },
        //         ContentType::Style => {
        //             manifest_styles.push(manifest_entry);
        //         },
        //     }
        // }

        let mut manifest_node_modules = Vec::new();

        for node_module in db.node_modules.iter() {
            // if let Some(id) = node_module.id {

            // let reference: ExternalNodeModuleReference = (&self.ctx,node_module).try_into()?;

            let manifest_entry = format!(
                "{} = {}",
                u64_to_hex_str(&node_module.id),
                node_module.to_string()
            );

            manifest_node_modules.push(manifest_entry);
            // }
        }

        let mut manifest_toml = String::new();
        const DO_NOT_EDIT: &str = "# es6 manifest - do not edit below this line";
        manifest_toml += &format!("\n\n\n{DO_NOT_EDIT}\n", );
        manifest_toml += &format!("# generated on {:?}\n\n", chrono::offset::Local::now());
        manifest_toml += &format!(
            "[manifest.node_modules]\n{}\n\n",
            manifest_node_modules.join("\n")
        );
        // manifest_toml += &format!("[manifest.modules]\n{}\n\n", manifest_modules.join("\n"));
        // manifest_toml += &format!("[manifest.scripts]\n{}\n\n", manifest_scripts.join("\n"));
        // manifest_toml += &format!("[manifest.styles]\n{}\n\n", manifest_styles.join("\n"));

        // let path_manifest_toml = self.ctx.target_folder.join("manifest.toml");
        // fs::write(&path_manifest_toml, &manifest_toml)?;

        let mut toml = fs::read_to_string(&self.ctx.manifest_toml)?;
        let lines = toml.split('\n').collect::<Vec<_>>();
        let index = lines.iter().position(|&l| l.contains(DO_NOT_EDIT));
        if let Some(index) = index {
            let mut slice = lines[0..index].to_vec();
            while !slice.is_empty() && slice[slice.len() - 1].trim().is_empty() {
                slice.remove(slice.len() - 1);
            }
            toml = slice.join("\n");
        }

        toml += &manifest_toml;
        // let toml = manifest_toml;
        fs::write(self.ctx.manifest_folder.join("es6.lib.toml"), toml)?;
        // fs::write(&self.ctx.manifest_toml, toml)?;

        Ok(())
    }

    // pub fn update_manifest2(&self, db : &Db, collection: &Collection) -> Result<()> {

    //     // let mut manifest_toml = String::new();
    //     // manifest_toml += "[manifest]\n\n\n";
    //     // let mut manifest_modules = Vec::new();
    //     // let mut manifest_scripts = Vec::new();
    //     // let mut manifest_styles = Vec::new();

    //     let mut modules = value::Table::new();
    //     let mut scripts = value::Table::new();
    //     let mut styles = value::Table::new();
    //     let mut node_modules = value::Table::new();

    //     for content in collection.content.iter() {
    //         let path = content.absolute.strip_prefix(&self.ctx.project_folder)?.to_str().unwrap().to_string();
    //         match content.content_type {
    //             ContentType::Module => {
    //                 modules.insert(content.id(),value::Value::String(path));
    //             },
    //             ContentType::Script => {
    //                 scripts.insert(content.id(),value::Value::String(path));
    //             },
    //             ContentType::Style => {
    //                 styles.insert(content.id(),value::Value::String(path));
    //             },
    //         }
    //     }

    //     for node_module in db.node_modules.iter() {
    //         if let Some(id) = node_module.id {
    //             node_modules.insert(format!("0x{:16x}",id),value::Value::String(node_module.name.clone()));
    //         }
    //     }

    //     let toml_text = fs::read_to_string(&self.ctx.manifest_toml)?;
    //     let mut toml: Value = match toml::from_str(&toml_text) {
    //         Ok(toml) => toml,
    //         Err(err) => {
    //             return Err(format!("Error loading es6.toml: {}", err).into());
    //         }
    //     };

    //     let manifest = toml.as_table_mut().unwrap();
    //     manifest.insert("manifest.modules".to_string(),value::Value::Table(modules));
    //     manifest.insert("manifest.scripts".to_string(),value::Value::Table(scripts));
    //     manifest.insert("manifest.styles".to_string(),value::Value::Table(styles));
    //     manifest.insert("manifest.node-modules".to_string(),value::Value::Table(node_modules));

    //     let toml_text = toml::to_string(manifest)?;
    //     // fs::write(&self.ctx.manifest_toml, toml_text)?;
    //     fs::write(&self.ctx.manifest_folder.join("es6-lib.toml"), toml_text)?;

    //     Ok(())
    // }
}
