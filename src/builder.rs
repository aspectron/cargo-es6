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
            println!("project file: `{}`", module.absolute.display());
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


    // pub async fn execute2(self: Arc<Builder>) -> Result<()> {

    //     let project_folder = &self.ctx.project_folder; //= Path::new("../flow-ux").canonicalize().await.unwrap();
    //     let target_file = &self.ctx.target_file;
    //     // let parent = Path::new("../").canonicalize().await.unwrap();
    //     // let test = parent.join("test");
    //     async_std::fs::create_dir_all(&self.ctx.target_folder).await.unwrap();
    //     // let target_file = test.join("flow-ux-unified.js");
    //     let include = Filter::new(&["*.js"]);
    //     let exclude = Filter::new(&["*/development/*","*/bundles/*"]);

    //     let list = WalkDir::new(&project_folder)
    //         .into_iter()
    //         .flatten()
    //         .filter_map(|entry|{
    //             let path = entry.path();
    //             let relative = path.strip_prefix(&project_folder).unwrap();
                
    //             let relative_str = relative.to_str().unwrap();
    //             if relative_str.len() == 0 || is_hidden(relative) {
    //                 return None;
    //             }

    //             if !include.is_match(relative_str) || !path.is_file() {
    //                 None
    //             } else if exclude.is_match(relative_str) {
    //                 // println!("ignore: `{}`",path.display());
    //                 None
    //             } else {
    //                 Some(Path::new(relative).to_path_buf())
    //             }
    //         });

    //     let mut files = Vec::new();
    //     let mut source_bytes: usize = 0;

    //     for file in list {
    //         let absolute = project_folder.join(&file);
    //         // let to_file = self.ctx.target_folder.join(&file);
    //         // println!("+{}",file.display());
    //         let text = async_std::fs::read_to_string(&absolute).await?;
    //         source_bytes += text.len();
    //         println!("{:>5}: `{}`",text.len(),absolute.display());
    //         files.push((absolute, text));
    //         // std::fs::copy(project_folder.join(&file),to_file)?;
    //     }

    //     println!("files: {} size: {:1.3} MB",files.len(), source_bytes as f64 / 1024.0 / 1024.0);
    //     println!("processing...");

    //     let mut modules = Vec::new();
    //     let mut modules_by_absolute = HashMap::new();
    //     let mut modules_by_name = HashMap::new();

    //     let mut dest = String::new();
    //     // let import_re = Regex::new(r###"import[^;]*from["'][^;]+["'];"###).unwrap();
    //     let import_re = Regex::new(r###"import[^;]*from\s*["']+[^"']+["']+;"###).unwrap();
    //     let import_reference_re = Regex::new(r###"import\s*(.+)\s*from\s*["'](.+)["']"###).unwrap();
    //     // let export_re = Regex::new(r"export[^;]+from[^;]+;").unwrap();
    //     for (absolute, text) in files.iter() {

    //         // let folder = project_folder.join(file);
    //         // let folder = folder.parent().unwrap();
    //         let import_matches = import_re.find_iter(text).map(|m| m.as_str().to_string()).collect::<Vec<_>>();

    //         println!("{}:",absolute.display());
    //         println!("{:#?}",import_matches);
    //         let mut imports = Vec::new();
    //         for import in import_matches.iter() {
    //             let import = import.replace("\n"," ");
    //             let captures = import_reference_re.captures(&import).expect(&format!("unable to capture `{}`",import));
    //             let what = captures[1].to_string();
    //             let filename = captures[2].to_string();
    //             println!("relative: {}",filename);
    //             let folder = if filename.starts_with("/") {
    //                 project_folder.clone()
    //             } else {
    //                 // project_folder.join(file).to_owned()
    //                 absolute.parent().unwrap().to_path_buf()
    //             };
    //             let filename = if filename.starts_with("./") {
    //                 filename[2..].to_string()
    //             } else { filename };
    //             let import = match folder.join(&filename).canonicalize().await {
    //                 Ok(path) => {
    //                     println!("absolute: {}",path.display());
    //                     Import { what, from : Reference::File(path.to_str().unwrap().to_string()) }
    //                 }, 
    //                 Err(e) => {
    //                     println!("error: {}",style(e).red());
    //                     Import { what, from : Reference::Module(filename) }
    //                 }
    //             };

    //             imports.push(import);
    //             // let filename = .expect(&format!("unable to canonicalize {}",filename));
    //         }
    //         let text = import_re.replace_all(text, "");
    //         // let text = export_re.replace_all(&text, "");
    //         dest += &format!("\n\n/* -> {} */\n\n {{ {} }}\n", absolute.display(), text);
    //         // dest += &text;

    //         let module = Arc::new(Module {
    // // absolute: file.clone(),
    //             absolute : absolute.to_str().unwrap().to_string(),
    //             file_name : absolute.file_name().unwrap().to_str().unwrap().to_string(),
    //             module_name : absolute.file_stem().unwrap().to_str().unwrap().to_string(),
    //             imports,
    //             content : text.to_string(),
    //             refs : Mutex::new(Vec::new())
    //             // imports: imports,
    //         });

    //         modules.push(module.clone());
    //         modules_by_absolute.insert(module.absolute.clone(), module.clone());
    //         modules_by_name.insert(module.module_name.clone(), module.clone());
    //     }


    //     for module in modules.iter() {

    //         if module.imports.is_empty() {
    //             continue;
    //         }

    //         for import in module.imports.iter() {
    //             match &import.from {
    //                 Reference::File(path) => {
    //                     let target = modules_by_absolute.get(path).expect(&format!("Unable to locate module `{}`", path));
    //                     println!("{}", module.absolute);
    //                     println!("\t{}", target.absolute);
    //                 },
    //                 Reference::Module(name) => {
    //                     let target = modules_by_name.get(name).expect(&format!("Unable to locate module `{}`", name));
    //                     println!("{}", module.module_name);
    //                     println!("\t{}", target.module_name);
    //                 }
    //             }
    //         }


    //     }

    //     // async_std::fs::write(target_file, dest).await.unwrap();

    //     Ok(())
    // }
}
