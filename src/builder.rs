use crate::prelude::*;

// mod filter;
// use filter::*;
// use async_std::path::{Path,PathBuf};
// use std::{collections::{HashSet,HashMap}, sync::{Arc, Mutex}};
// use walkdir::WalkDir;
// use regex::Regex;
// use console::style;

// pub enum Reference {
//     File(String),
//     Module(String),
// }

// pub struct Import {
//     what : String,
//     from : Reference,
// }
// pub struct Module {
//     pub absolute : String,
//     pub content : String,
//     pub file_name : String,
//     pub module_name : String,
//     pub imports : Vec<Import>,
//     pub refs : Mutex<Vec<Arc<Module>>>,
// }

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
            self.generate(&module).await?;
        } else {
            return Err(format!("Unabel to resolve project file: `{}`", self.ctx.project_file.display()).into());
        }


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
            let target = modules.resolve(&import.location,&import.referrer).await?;
            *import.reference.lock().unwrap() = target;//.clone();
        }

        for export in module.exports.iter() {
            let target = modules.resolve(&export.location,&export.referrer).await?;
            *export.reference.lock().unwrap() = target;//.clone();
        }


        Ok(())
    }

    pub async fn generate(self: &Arc<Builder>, module : &Arc<FileModule>) -> Result<()> {

        log_info!("Generating","`{}`",self.ctx.target_file.display());
        
        let mut collection = Collection::new();
        module.gather(&mut collection)?;
        
        let mut text = String::new();
        for module in collection.modules.iter() {
            // println!("{}",module.ident);

            //   

            text += &format!("const {}_{} : &'static str = r###\"\n", module.ident, module.id);
            // TODO - IMPORTS
            text += &module.content;
            // TODO - EXPORTS
            text += &format!("\n\"###;\n\n");

        }

        async_std::fs::write(&self.ctx.target_file, &text).await?;
        let file_size = std::fs::metadata(&self.ctx.target_file)?.len() as f64 / 1024.0;
        log_info!("Generating","... modules: {} file size: {:1.0} Kb", collection.modules.len(), file_size);

        Ok(())
    }

}
