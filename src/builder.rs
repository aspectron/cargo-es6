use crate::prelude::*;

pub struct Builder {
    ctx : Arc<Context>
}

impl Builder {

    pub fn new(ctx : Arc<Context>) -> Builder {
        Builder {
            ctx
        }
    }

    pub async fn execute(self: Arc<Builder>) -> Result<()> {

        if !self.ctx.node_modules.exists() {
            cmd!("npm install").dir(&self.ctx.project_folder).run()?;
        }

        async_std::fs::create_dir_all(&self.ctx.target_folder).await.unwrap();

        let mut modules = Db::load(&self.ctx).await?;
        for node_module in modules.node_modules.iter() {
            log_info!("Module","`{}` files: {} explicit exports: {}", style(&node_module.name).cyan(), node_module.files.len(), node_module.exports.len());
        }

        self.resolve(&mut modules).await?;

        println!("");

        let module = modules.file_content_by_absolute.get(&self.ctx.project_file);
        if let Some(module) = module {
            self.generate(&module, &modules).await?;
        } else {
            return Err(format!("Unabel to resolve project file: `{}`", self.ctx.project_file.display()).into());
        }

        self.build_wasm().await?;

        Ok(())
    }

    pub async fn resolve(self: &Arc<Builder>, db: &mut Db) -> Result<()> {

        let file_content = db.file_content.clone();
        for content in file_content.iter() {
            content.resolve(db)?;
            // self.resolve_module(module,modules).await?;
        }

        Ok(())
    }

    pub async fn get_references(self: &Arc<Builder>, files : &Vec<String>, modules: &Db) -> Result<Vec<Arc<Content>>> {
        let mut targets = Vec::new();
        for file in files.iter() {
            let in_root = self.ctx.project_folder.join(file);
            if in_root.canonicalize().is_ok() {
                targets.push(in_root);
            } else {
                let in_node_modules = self.ctx.node_modules.join(file);
                if in_node_modules.canonicalize().is_ok() {
                    targets.push(in_node_modules);
                } else {
                    return Err(format!("get_references(): unable to locate `{}` in project root or node modules", file).into());
                }
            }
        }


        let mut references = Vec::new();
        for target in targets.iter() {
            let module = modules.file_content_by_absolute.get(target); //modules.resolve(location, referrer)
            if let Some(module) = module {

                let list = module.references();
                references.extend_from_slice(&list);
                // let mut components = Vec::new();
                // for reference in references.iter() {
                //     // components.push(reference.clone());

                // }
                // references.iter().map(|reference| {}).collect::

            } else {
                return Err(format!("get_references(): unable to resolve module `{}`",target.display()).into());
            }
        }

        Ok(references)
    }

    pub async fn generate(self: &Arc<Builder>, root_module : &Arc<Content>, db: &Db) -> Result<()> {
// std::process::exit(1);
        let module_id_repr = "u64";

        let enums = if let Some(enums) = &self.ctx.manifest.settings.enums {
            let references = self.get_references(&enums.exports, db).await?;
            let mut text = String::new();
            text.push_str(&format!("\n#[allow(dead_code)]\n#[repr({})]\npub enum Modules {{\n", module_id_repr));
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
        log_info!("Generating","`{}`",self.ctx.target_folder.display());
        // let ident_kind = IdentKind::HexFull;
        let ident_kind = IdentKind::Full;
        
        let mut collection = Collection::new();
        root_module.gather(&mut collection)?;
        
        let mut manifest_toml = String::new();
        // let mut manifest_toml_scripts = String::new();
        // let mut manifest_toml_modules = String::new();
        // let mut manifest_toml_styles = String::new();
        manifest_toml += "[manifest]\n\n\n";
        // manifest_toml_scripts += "\n[scripts]\n";
        // manifest_toml_modules += "\n[modules]\n";
        // manifest_toml_styles += "\n[styles]\n";
        let mut manifest_modules = Vec::new();
        let mut manifest_scripts = Vec::new();
        let mut manifest_styles = Vec::new();

        let mut content_rs = String::new();
        for content in collection.content.iter() {
            // println!("{}",module.ident);            
            content_rs += &format!("pub const {} : &'static str = r###\"\n", content.ident(&ident_kind));
            // TODO - IMPORTS
            content_rs += &content.content;
            // TODO - EXPORTS
            content_rs += &format!("\n\"###;\n\n");
            
            // ~~~

            let manifest_entry = format!("{} = \"{}\"", 
                // content.content_type.to_manifest_type(),
                content.id(),
                content.absolute.strip_prefix(&self.ctx.project_folder)?.to_str().unwrap()
            );

            match content.content_type {
                ContentType::Module => {
                    manifest_modules.push(manifest_entry);
                },
                ContentType::Script => {
                    manifest_scripts.push(manifest_entry);
                },
                ContentType::Style => {
                    manifest_styles.push(manifest_entry);
                },
            }
        }

        let mut manifest_node_modules = Vec::new();

        for node_module in db.node_modules.iter() {
            if let Some(id) = node_module.id {
                let manifest_entry = format!("0x{:16x} = \"{}\"", 
                    id,
                    node_module.name
                );

                manifest_node_modules.push(manifest_entry);
            }
        }
        
        manifest_toml += &format!("[node_modules]\n{}\n\n", manifest_node_modules.join("\n"));
        manifest_toml += &format!("[modules]\n{}\n\n", manifest_modules.join("\n"));
        manifest_toml += &format!("[scripts]\n{}\n\n", manifest_scripts.join("\n"));
        manifest_toml += &format!("[styles]\n{}\n\n", manifest_styles.join("\n"));

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
        context_rs += &format!("\nconst ROOT: Id = {};\n",root_module.id());

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
            context_rs = context_rs.replace("log_info","// log_info");
        }

        context_rs += &enums;

        let mut table = Vec::new();
        for content in collection.content.iter() {

            let mut references = Vec::new();
            if let Some(targets) = &content.references {
                for reference in targets.iter() {
                    match reference.content() {
                        Some(content) => {
                            match reference.kind {
                                ReferenceKind::Style => {
                                    references.push(format!("(Reference::Style,None,{})",content.id()));
                                },
                                ReferenceKind::Module => {
                                    if let Some(what) = &reference.what {
                                        references.push(format!("(Reference::Module,Some(\"{}\"),{})",what,content.id()));
                                    } else {
                                        references.push(format!("(Reference::Module,None,{})",content.id()));
                                    }
                                },
                                ReferenceKind::Script => {
                                    references.push(format!("(Reference::Script,None,{})",content.id()));
                                },
                                ReferenceKind::Export => {
                                    references.push(format!("(Reference::Export,Some(\"{}\"),{})",reference.what.as_ref().unwrap(),content.id()));
                                }
                            }
                        },
                        None => {
                            reference.error();
                            // .expect(&format!("failure dereferencing `{}` -> `{}`", reference.referrer.display(),reference.location));
                        }
                    }

                    

                }
            }

            let references = if references.is_empty() {
                "None".to_string()
            } else {
                format!("Some(&[\n\t\t{}\n\t])",references.join(",\n\t\t"))
            };

            let definition = format!("Arc::new(Content {{\n\
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

            table.push(format!("(0x{:x},{})",content.id,definition));
        }
        let table = table.join(",\n");
        context_rs += r###"
        impl Context {
            pub fn new() -> Self {

        "###;
        // context_rs += &format!("\nlet content : ContentList = [\n{}\n].into_iter().collect();\n", table);
        context_rs += &format!("\nlet content : ContentList = &[\n{}\n];\n", table);
        context_rs += r###"
                Context {
                    inner : declare(content)
                }
            }
        }
        "###;

        
        let path_manifest_toml = self.ctx.target_folder.join("manifest.toml");
        let path_lib_rs = self.ctx.target_folder.join("mod.rs");
        let path_content_rs = self.ctx.target_folder.join("content.rs");
        let path_modules_rs = self.ctx.target_folder.join("context.rs");
        async_std::fs::write(&path_manifest_toml, &manifest_toml).await?;
        async_std::fs::write(&path_lib_rs, &lib_rs).await?;
        async_std::fs::write(&path_content_rs, &content_rs).await?;
        async_std::fs::write(&path_modules_rs, &context_rs).await?;
        let mut file_size = 0.0;
        // file_size += std::fs::metadata(&path_lib_rs)?.len() as f64 / 1024.0;
        file_size += std::fs::metadata(&path_content_rs)?.len() as f64 / 1024.0;
        // file_size += std::fs::metadata(&path_modules_rs)?.len() as f64 / 1024.0;
        log_info!("Generating","... modules: {} content file size: {:1.0} Kb", collection.content.len(), file_size);

        Ok(())
    }

    pub async fn build_wasm(&self) -> Result<()> {

        if let Some(wasm) = &self.ctx.manifest.settings.wasm {

            // wasm-pack build --dev --target web --out-name $NAME --out-dir root/wasm
            // wasm-pack build --target web --out-name $NAME --out-dir root/wasm

            let folder = self.ctx.project_folder.join(&wasm.folder).canonicalize()?;

            let outdir = folder.join(&wasm.outdir);//.join(wasm.name);
            let outdir = outdir.to_str().unwrap();

            let mut args = Vec::new();
            args.push("build");
            if wasm.dev.unwrap_or(false) {
                args.push("--dev");
            }
            args.extend_from_slice(&["--target","web","--out-name",&wasm.name,"--out-dir",outdir]);

            cmd("wasm-pack",&args).dir(folder).run()?;
        }

        Ok(())
    }
}
