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


        if !self.ctx.node_modules.exists().await {
            cmd!("npm install").dir(&self.ctx.project_folder).run()?;
        }

        async_std::fs::create_dir_all(&self.ctx.target_folder).await.unwrap();

        let modules = Modules::load(&self.ctx).await?;

        // for module in node_modules.modules.iter() {
        //     println!("{}", module.absolute.display());
        //     println!("name: `{}` files: {} exports: {}", module.name, module.files.len(), module.exports.len());
        // }

        for node_module in modules.node_modules.iter() {
            // println!("{}", module.absolute.display());
            log_info!("Module","`{}` files: {} explicit exports: {}", style(&node_module.name).cyan(), node_module.files.len(), node_module.exports.len());
        }

        self.resolve(&modules).await?;


        println!("");

        let module = modules.file_modules_by_absolute.get(&self.ctx.project_file);
        if let Some(module) = module {
            self.generate(&module, &modules).await?;
        } else {
            return Err(format!("Unabel to resolve project file: `{}`", self.ctx.project_file.display()).into());
        }

        self.build_wasm().await?;

        // let xx = modules.node_modules_by_name.get("flow-ux");
        // println!("{:?}", xx);
        
        Ok(())
    }

    pub async fn resolve(self: &Arc<Builder>, modules: &Modules) -> Result<()> {


        for module in modules.file_modules.iter() {

            // println!("... resolving: `{}`", module.absolute.display());
            // for module in node_module.files.iter() {
                self.resolve_module(module,modules).await?;
            // }

        }

        Ok(())
    }

    pub async fn resolve_module(self: &Arc<Builder>, module: &FileModule, modules: &Modules) -> Result<()> {

        for import in module.imports.iter() {
            match modules.resolve(&import.location,&import.referrer).await {
                Ok(target) => {
                    *import.reference.lock().unwrap() = target;//.clone();
                },
                Err(_err) => {
                    let relative = import.referrer.strip_prefix(&self.ctx.project_folder)?;
                    if !self.ctx.ignore.is_match(&relative.to_string_lossy()) && !self.ctx.ignore.is_match(&import.location) {
                        log_warn!("Resolver","referrer: `{}`",import.referrer.display());
                        log_warn!("","target: `{}`",import.location);
                    }
                    // import.warn();
                    // log_error!("{}",err);
                }
            }
        }

        for export in module.exports.iter() {
            match modules.resolve(&export.location,&export.referrer).await {
                Ok(target) => {
                    *export.reference.lock().unwrap() = target;
                },
                Err(_err) => {
                    log_warn!("Export","`{}`",export.referrer.display());
                    log_warn!("Target","`{}`",export.location);
                    // export.warn();
                    // log_error!("{}",err);
                }
            }
        }


        Ok(())
    }

    // pub async fn get_references(self: &Arc<Builder>, enums : &Enums, modules: &Modules) -> Result<Vec<Arc<FileModule>>> {
    pub async fn get_references(self: &Arc<Builder>, files : &Vec<String>, modules: &Modules) -> Result<Vec<Arc<FileModule>>> {
        // let text = String::new();

        // let exports = &enums.exports;

        let mut targets = Vec::new();
        for file in files.iter() {
            let in_root = self.ctx.project_folder.join(file);
            if in_root.canonicalize().await.is_ok() {
                targets.push(in_root);
            } else {
                let in_node_modules = self.ctx.node_modules.join(file);
                if in_node_modules.canonicalize().await.is_ok() {
                    targets.push(in_node_modules);
                } else {
                    return Err(format!("get_references(): unable to locate `{}` in project root or node modules", file).into());
                }
            }
        }


        let mut references = Vec::new();
        for target in targets.iter() {
            let module = modules.file_modules_by_absolute.get(target); //modules.resolve(location, referrer)
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

    pub async fn generate(self: &Arc<Builder>, root_module : &Arc<FileModule>, modules: &Modules) -> Result<()> {

        let module_id_repr = "u64";

        let enums = if let Some(enums) = &self.ctx.manifest.settings.enums {
            let references = self.get_references(&enums.exports, modules).await?;
            let mut text = String::new();
            text.push_str(&format!("\n#[allow(dead_code)]\n#[repr({})]\npub enum Modules {{\n", module_id_repr));
            text.push_str("\tAll = 0,\n");
            for module in references.iter() {
                text.push_str(&format!("\t{} = {},\n", module.component, module.id));
            }
            text.push_str("}\n");
            text
        } else {
            String::new()
        };

        // log_info!("Generating","`{}`",self.ctx.target_file.display());
        log_info!("Generating","`{}`",self.ctx.target_folder.display());
        // let ident_kind = IdentKind::HexFull;
        let ident_kind = IdentKind::IntegerFull;
        
        let mut collection = Collection::new();
        root_module.gather(&mut collection)?;
        
        let mut content_rs = String::new();
        for module in collection.modules.iter() {
            // println!("{}",module.ident);            
            content_rs += &format!("pub const {} : &'static str = r###\"\n", module.ident(&ident_kind));
            // TODO - IMPORTS
            content_rs += &module.content;
            // TODO - EXPORTS
            content_rs += &format!("\n\"###;\n\n");
            
        }
        

        let lib_rs = r###"
use wasm_bindgen::prelude::*;
mod content;
mod modules;
pub use modules::*;

#[wasm_bindgen]
pub async fn load_modules() -> modules::Result<()> {
    workflow_wasm::panic::init_console_panic_hook();
    let ctx = modules::Context::new();
    ctx.load_all().await?;
    Ok(())
}
"###;

        let mut modules_rs = String::new();
        modules_rs += r###"
        use std::sync::{
            Arc,
            Mutex,
            atomic::AtomicBool
        };
        use workflow_dom::loader::{
            // Module,
            ModuleMap,
            Id,
            Reference,
            Context as Inner
        };
        pub use workflow_dom::loader::Module;
        pub use workflow_dom::result::Result;
        pub use workflow_dom::error::Error;
        use crate::content;

        "###;
        modules_rs += &format!("\nconst ROOT: Id = {};\n",root_module.id);

        modules_rs += r###"
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
            
        }
        "###;
        // modules_rs += &format!("\npub type ModuleId = {};\n", module_id_repr);
        
        modules_rs += r###"

"###;

        if !self.ctx.manifest.settings.verbose.unwrap_or(false) {
            modules_rs = modules_rs.replace("log_info","// log_info");
        }

        modules_rs += &enums;

        let mut table = Vec::new();
        for module in collection.modules.iter() {

            let mut references = Vec::new();
            for import in module.imports.iter() {
                let reference = import
                    .reference()
                    .expect(&format!("failure dereferencing `{}` -> `{}`", import.referrer.display(),import.location));
                
                
                match import.kind {
                    ReferenceKind::Style => {
                        references.push(format!("(Reference::Style,None,{})",reference.id));
                    },
                    ReferenceKind::Module => {
                        if let Some(what) = &import.what {
                            references.push(format!("(Reference::Module,Some(\"{}\"),{})",what,reference.id));
                        } else {
                            references.push(format!("(Reference::Module,None,{})",reference.id));
                        }
                    },
                    _ => {
                        panic!("wtf");
                    }
                }
            }
            // let references = references.join(",");
            
            // let mut exports = Vec::new();
            for export in module.exports.iter() {
                let reference = export.reference().unwrap();
                references.push(format!("(Reference::Export,Some(\"{}\"),{})",export.what.as_ref().unwrap(),reference.id));
            }
            let references = references.join(",");

            // println!("{}",module.ident);            
            // text += &format!("\n\t\t\tconst MODULE_{}: Module = Module {{\n\
                // \timports : &[{}],\n\
                // \texports : &[{}],\n\
            let definition = format!("Arc::new(Module {{\n\
                \turl : Mutex::new(None),\n\
                \tid : {},\n\
                \tident : \"{}\",\n\
                \tcontent : content::{},\n\
                \treferences : &[{}],\n\
                \tis_loaded : AtomicBool::new(false),\n\
            }})", 
                // module.id(&ident_kind),
                module.id,
                module.ident(&ident_kind).to_lowercase(),
                module.ident(&ident_kind),
                references,
                // imports,
                // exports,
            );

            table.push(format!("({},{})",module.id,definition));
        }
        // let table_len = table.len();
        let table = table.join(",\n");
        modules_rs += r###"
        impl Context {
            pub fn new() -> Self {

        "###;

        // text += &format!("\nlet modules : [(ModuleId,Arc<Module>);{}] = [\n{}\n];\n",table_len, table);
        modules_rs += &format!("\nlet modules : ModuleMap = [\n{}\n].into_iter().collect();\n", table);

        modules_rs += r###"
                Context {
                    inner : Arc::new(Inner::new(modules))
                }
            }
        }
        "###;
        

        // let mut groups = Vec::new();
        // let mut comments = Vec::new();
        // for group in collection.groups.iter() {
        //     let ids = group.iter().map(|m|m.id.to_string()).collect::<Vec<_>>().join(", ");
        //     let names = group.iter().map(|m|m.ident.clone()).collect::<Vec<_>>().join(", ");
        //     groups.push(format!("&[{}]",ids));
        //     comments.push(names)
        // }
        // let comments = comments.join("\n// ");
        // let groups = groups.join(",\n\t");

        // text += &format!("\n// {}\n", comments);
        // text += &format!("\nconst DEPENDENCIES : &[&[ModuleId]] = &[\n\t{}\n];\n", groups);

        // text += r###"
        // }
        // "###;

        let path_lib_rs = self.ctx.target_folder_src.join("lib.rs");
        let path_content_rs = self.ctx.target_folder_src.join("content.rs");
        let path_modules_rs = self.ctx.target_folder_src.join("modules.rs");
        async_std::fs::write(&path_lib_rs, &lib_rs).await?;
        async_std::fs::write(&path_content_rs, &content_rs).await?;
        async_std::fs::write(&path_modules_rs, &modules_rs).await?;
        let mut file_size = 0.0;
        // file_size += std::fs::metadata(&path_lib_rs)?.len() as f64 / 1024.0;
        file_size += std::fs::metadata(&path_content_rs)?.len() as f64 / 1024.0;
        // file_size += std::fs::metadata(&path_modules_rs)?.len() as f64 / 1024.0;
        log_info!("Generating","... modules: {} file size: {:1.0} Kb", collection.modules.len(), file_size);

        Ok(())
    }

    pub async fn build_wasm(&self) -> Result<()> {

        if let Some(wasm) = &self.ctx.manifest.settings.wasm {

            // wasm-pack build --dev --target web --out-name $NAME --out-dir root/wasm
            // wasm-pack build --target web --out-name $NAME --out-dir root/wasm

            let folder = self.ctx.project_folder.join(&wasm.folder).canonicalize().await?;

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
