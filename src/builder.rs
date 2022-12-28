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

        log_info!("Generating","`{}`",self.ctx.target_file.display());
        // let ident_kind = IdentKind::HexFull;
        let ident_kind = IdentKind::IntegerFull;
        
        let mut collection = Collection::new();
        root_module.gather(&mut collection)?;
        
        let mut text = String::new();
        for module in collection.modules.iter() {
            // println!("{}",module.ident);            
            text += &format!("const {} : &'static str = r###\"\n", module.ident(&ident_kind));
            // TODO - IMPORTS
            text += &module.content;
            // TODO - EXPORTS
            text += &format!("\n\"###;\n\n");

        }

        text += r###"
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use workflow_log::*;
use workflow_wasm::callback::*;
use js_sys::{Array,Uint8Array};
use wasm_bindgen::prelude::*;
// use wasm_bindgen::prelude::*;
use web_sys::{Document,Url,Blob};
// use crate::result::*;
// use crate::utils::*;
use workflow_core::channel::oneshot;
// use thiserror::Error;
use futures::future::{join_all,BoxFuture,FutureExt};
// use futures::future::{join_all,ok,err,BoxFuture,FutureExt};

use wasm_bindgen::JsCast;


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    String(String),
    #[error("{0:?}")]
    JsValue(JsValue),
}
unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl From<String> for Error { fn from(v:String) -> Self { Self::String(v) } }
impl From<&str> for Error { fn from(v:&str) -> Self { Self::String(v.to_string()) } }
impl From<JsValue> for Error { fn from(v:JsValue) -> Self { Self::JsValue(v) } }
// impl From<JsValue> for Error { fn from(_v:JsValue) -> Self { Self::String("JsValue error".into()) } }

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;


pub fn document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

pub fn root() -> web_sys::Element {
    let collection = document().get_elements_by_tag_name("head");
    if collection.length() > 0 {
        collection.item(0).unwrap()
    } else {
        document().get_elements_by_tag_name("body").item(0).unwrap()
    }
}

#[allow(dead_code)]
pub enum Reference {
    Style,
    Import,
    Export,
}

#[allow(dead_code)]
pub enum ModuleStatus {
    Loaded,
    Exists,
    Error,
}

// unsafe impl Send for ModuleStatus {}
// unsafe impl Sync for ModuleStatus {}

"###;
text += &format!("\npub type ModuleId = {};\n", module_id_repr);
text += &format!("\nconst ROOT: ModuleId = {};\n",root_module.id);
text += r###"
pub struct Module {
    url : Mutex<Option<String>>,
    ident : &'static str,
    content: &'static str,
    references: &'static [(Reference, Option<&'static str>, ModuleId)],
    // imports: &'static [(Reference, Option<&'static str>, ModuleId)],
    // exports: &'static [(&'static str, ModuleId)],
    is_loaded : AtomicBool,
}

impl Module {

    pub fn url(&self) -> Option<String> { 
        self.url.lock().unwrap().clone() 
    }

    pub fn content(&self, modules: &ModuleMap) -> Result<String> {
        let mut text = String::new();

        let mut imports = Vec::new();
        let mut exports = Vec::new();

        for (kind,what,id) in self.references.iter() {
            let module = modules.get(id).ok_or(format!("unable to lookup module `{}`",self.ident))?;
            let url = module.url().ok_or(format!("[{}] module is not loaded `{}`",self.ident,id))?;
            match kind {
                Reference::Style => {
                    // TODO: import style into the global document
                    log_info!("TODO: loading style {}", what.unwrap())
                },
                Reference::Import => {
                    match what {
                        Some(detail) => {
                            imports.push(format!("import {} from \"{}\";\n", detail, url));
                        },
                        None => {
                            imports.push(format!("import \"{}\";\n", url));
                        }
                    }
                },
                Reference::Export => {
                    let module = modules.get(id).ok_or(format!("unable to lookup module `{}`",self.ident))?;
                    let url = module.url().ok_or(format!("[{}] module is not loaded `{}`",self.ident,id))?;
                    exports.push(format!("export {} from \"{}\";\n", what.unwrap(), url));
                }
            }
        }
        let imports = imports.join("\n");

        // let mut exports = Vec::new();
        // for (what,id) in self.exports.iter() {
        //     let module = modules.get(id).ok_or(format!("unable to lookup module `{}`",self.ident))?;
        //     let url = module.url().ok_or(format!("[{}] module is not loaded `{}`",self.ident,id))?;
        //     exports.push(format!("export {} from \"{}\";\n", what, url));
        // }
        let exports = exports.join("\n");

        text += &imports;
        text += &self.content;
        text += &exports;

        Ok(text)
    }

    fn is_loaded(&self) -> bool {
        self.is_loaded.load(Ordering::SeqCst)
    }

    pub fn load_deps(self : Arc<Self>, modules: Arc<ModuleMap>) -> BoxFuture<'static, Result<()>> {

        // if self.is_loaded.load(Ordering::SeqCst) {
        //     return None;
        // }

        // let future = 
        async move {
            // log_info!("[{}] processing dependencies...", self.ident);
            let futures = self.references
                .iter()
                .map(|(kind,what,id)| {
                    match kind {
                        Reference::Import | Reference::Export => {
                            if let Some(module) = modules.get(id) {
                                if !module.is_loaded.load(Ordering::SeqCst) {
                                    // log_info!("[inner] loading dependency ... {}", id);
                                    Some(module.load(&modules))
                                } else {
                                    // log_info!("module already loaded {}", id);
                                    None
                                }
                            } else {
                                log_error!("Unable to locate module {}", id);
                                // TODO: panic
                                None
                            }
                        },
                        Reference::Style => {
                            log_info!("TODO: import stylesheet: {:?}",what);
                            None
                        }
                    }
                })
                .flatten()
                .collect::<Vec<_>>();
            
            // log_info!("{} waiting for dependencies to load...",self.ident);
            // TODO: use Join

            // join_all(futures).await;

            for future in futures {
                match future.await {
                    Ok(_event) => {
                        
                    },
                    Err(err) => {
                        log_error!("{}", err);
                    }
                }
            }

            Ok(())
        }.boxed()

        // Some(future)

        // Ok(())
    }

    pub async fn load(self : &Arc<Self>, modules: &Arc<ModuleMap>) -> Result<ModuleStatus> {

        // if let Some(future) =  self.clone().load_deps(modules){
        //     future.await?;
        // }
        
        if self.is_loaded() {
            // log_info!("{} already loaded ...", self.ident);
            return Ok(ModuleStatus::Exists);
        }
        
        // log_info!("{} loading dependencies ...", self.ident);
        self.clone().load_deps(modules.clone()).await?;
        // log_info!("{} dependencies load done ...", self.ident);
        
        log_info!("load ... {}", self.ident);
        
        let (sender,receiver) = oneshot();
        // let cb = 
        {
            let content = self.content(modules)?;
            let args = Array::new_with_length(1);
            args.set(0, unsafe { Uint8Array::view(content.as_bytes()).into() });
            let mut options = web_sys::BlobPropertyBag::new();
            // options.type_("module");
            options.type_("application/javascript");
            let blob = Blob::new_with_u8_array_sequence_and_options(&args, &options)?;
            let url = Url::create_object_url_with_blob(&blob)?;
            self.url.lock().unwrap().replace(url.clone());
            
            let script = document().create_element("script")?;
            // if let Some(closure) = load {
                
            // let ident = self.ident.clone();

            let closure = Closure::<dyn FnMut(web_sys::CustomEvent)->std::result::Result<(), JsValue>>::new(move|_event|->std::result::Result<(), JsValue>{
                // TODO
                let status = ModuleStatus::Loaded;
                sender.try_send(status).expect("unable to post load event");

                Ok(())
            });
            script.add_event_listener_with_callback("load", closure.as_ref().unchecked_ref())?;
            closure.forget();

            // let callback = callback!(move |_event: web_sys::CustomEvent| {
            //     log_info!("{} ... done", ident);
            //     // TODO
            //     let status = ModuleStatus::Loaded;
            //     sender.try_send(status).expect("unable to post load event");
            //     // drop(callback);
            // });
            // script.add_event_listener_with_callback("load", callback.as_ref())?;

            let content_type = "module";
            script.set_attribute("module","true")?;
            script.set_attribute("type",content_type)?;
            script.set_attribute("src", &url)?;
            root().append_child(&script)?;

            // Arc::new(Mutex::new(callback))
        }
        let status = receiver.recv().await.expect("unable to recv() load event");
        self.is_loaded.store(true, Ordering::SeqCst);
        Ok(status)
    }
}

pub type ModuleMap = HashMap<ModuleId,Arc<Module>>;

// #[wasm_bindgen]
// pub async fn load_modules() -> Result<()> {
// }


pub struct Context {
    pub modules : Arc<ModuleMap>
}

impl Context {
    pub async fn load_all(&self) -> Result<()> {
        self.load_modules(&[ROOT]).await?;
        Ok(())
    }

    pub async fn load_modules(&self, list : &[ModuleId]) -> Result<()> {

        // let modules : ModuleMap = MODULES.into_iter().collect();
        
        let futures = list
            .iter()
            .map(|id| {
                if let Some(module) = self.modules.get(id) {
                    Some(module.load(&self.modules))
                } else {
                    log_error!("Unable to locate module {}", id);
                    // TODO: panic
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        
        for future in futures {
            match future.await {
                Ok(_event) => {

                },
                Err(err) => {
                    log_error!("{}", err);
                }
            }
        }

        Ok(())
    }
}

"###;

        if !self.ctx.manifest.settings.verbose.unwrap_or(false) {
            text = text.replace("log_info","// log_info");
        }

        text += &enums;

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
                    ReferenceKind::Import => {
                        if let Some(what) = &import.what {
                            references.push(format!("(Reference::Import,Some(\"{}\"),{})",what,reference.id));
                        } else {
                            references.push(format!("(Reference::Import,None,{})",reference.id));
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
                \tident : \"{}\",\n\
                \tcontent : {},\n\
                \treferences : &[{}],\n\
                \tis_loaded : AtomicBool::new(false),\n\
            }})", 
                // module.id(&ident_kind),
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
        text += r###"
        impl Default for Context {
            fn default() -> Self {

        "###;

        // text += &format!("\nlet modules : [(ModuleId,Arc<Module>);{}] = [\n{}\n];\n",table_len, table);
        text += &format!("\nlet modules : ModuleMap = [\n{}\n].into_iter().collect();\n", table);

        text += r###"

                Context { modules : Arc::new(modules) }
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

        async_std::fs::write(&self.ctx.target_file, &text).await?;
        let file_size = std::fs::metadata(&self.ctx.target_file)?.len() as f64 / 1024.0;
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
