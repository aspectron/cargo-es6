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
            self.generate(&module).await?;
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
                    log_warn!("Resolver","referrer: `{}`",import.referrer.display());
                    log_warn!("","target: `{}`",import.location);
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

    pub async fn generate(self: &Arc<Builder>, module : &Arc<FileModule>) -> Result<()> {

        log_info!("Generating","`{}`",self.ctx.target_file.display());
        // let ident_kind = IdentKind::HexFull;
        let ident_kind = IdentKind::IntegerFull;
        
        let mut collection = Collection::new();
        module.gather(&mut collection)?;
        
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
        // use std::sync::Arc;
        use std::sync::Mutex;
        use workflow_log::*;
        use workflow_wasm::callback::*;
        use js_sys::{Array,Uint8Array};
        use wasm_bindgen::prelude::*;
        // use wasm_bindgen::prelude::*;
        use web_sys::{Document,Element,Url,Blob};
        // use crate::result::*;
        // use crate::utils::*;
        use workflow_core::channel::oneshot;
        // use thiserror::Error;

        #[derive(thiserror::Error, Debug)]
        pub enum Error {
            #[error("{0}")]
            String(String),
			#[error("{0:?}")]
            JsValue(JsValue),
        }

        impl From<String> for Error { fn from(v:String) -> Self { Self::String(v) } }
        impl From<&str> for Error { fn from(v:&str) -> Self { Self::String(v.to_string()) } }
        impl From<JsValue> for Error { fn from(v:JsValue) -> Self { Self::JsValue(v) } }

        impl Into<JsValue> for Error {
            fn into(self) -> JsValue {
                JsValue::from_str(&self.to_string())
            }
        }

        pub type Result<T> = std::result::Result<T, Error>;

        pub fn document() -> Document {
            web_sys::window().unwrap().document().unwrap()
        }

        pub enum Import {
            All,
            Detail(&'static str)
        }

        pub type ModuleId = u64;
        pub struct Module {
            url : Mutex<Option<String>>,
            ident : &'static str,
            content: &'static str,
            imports: &'static [(Import, ModuleId)],
            // imports: &'static [(&'static str, ModuleId)],
            exports: &'static [(&'static str, ModuleId)],
        }

        impl Module {

            pub fn url(&self) -> Option<String> { 
                self.url.lock().unwrap().clone() 
            }

            pub fn content(&self, modules: &ModuleMap) -> Result<String> {
                let mut text = String::new();

                let mut imports = Vec::new();
                for (what,id) in self.imports.iter() {
                    let module = modules.get(id).ok_or(format!("unable to lookup module `{}`",self.ident))?;
                    let url = module.url().ok_or(format!("[{}] module is not loaded `{}`",self.ident,id))?;
                    match what {
                        Import::Detail(detail) => {
                            imports.push(format!("import {} from \"{}\";\n", detail, url));
                        },
                        Import::All => {
                            imports.push(format!("import \"{}\";\n", url));
                        }
                    }
                }
                let imports = imports.join("\n");

                let mut exports = Vec::new();
                for (what,id) in self.exports.iter() {
                    let module = modules.get(id).ok_or(format!("unable to lookup module `{}`",self.ident))?;
                    let url = module.url().ok_or(format!("[{}] module is not loaded `{}`",self.ident,id))?;
                    exports.push(format!("export {} from \"{}\";\n", what, url));
                }
                let exports = exports.join("\n");

                // let imports = self.imports.iter().map(|(what, id)| {
                //     let url = modules
                //         .get(id)
                //         .expect(&)
                //         .url()
                //         .expect(&format!("module is not loaded `{}`",self.ident));
                //     format!("import {} from \"{}\";\n", what, url)
                // }).collect::<Vec<_>>().join("\n");

                // let exports = self.exports.iter().map(|(what, id)| {
                //     let url = modules
                //         .get(id)
                //         .expect(&format!("unable to lookup module `{}`",self.ident))
                //         .url()
                //         .expect(&format!("module is not loaded `{}`",self.ident));
                //     format!("export {} from \"{}\";\n", what, url)
                // }).collect::<Vec<_>>().join("\n");

                text += &imports;
                text += &self.content;
                text += &exports;

                Ok(text)
            }

            pub async fn load(&self, root: &Element, modules: &ModuleMap) -> Result<web_sys::CustomEvent>{

                log_info!("{} load ...", self.ident);

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
                    
                let ident = self.ident.clone();
                let (sender,receiver) = oneshot();
                let callback = callback!(move |event: web_sys::CustomEvent| {
                    log_info!("{} ... done", ident);
                    sender.try_send(event).expect("unable to post load event");
                });
                script.add_event_listener_with_callback("load", callback.as_ref())?;
                
                let content_type = "module";
                script.set_attribute("module","true")?;
                script.set_attribute("type",content_type)?;
                script.set_attribute("src", &url)?;
                root.append_child(&script)?;
                let event = receiver.recv().await.expect("unable to recv() load event");
                Ok(event)
            }
        }

        pub type ModuleMap = HashMap<ModuleId, Module>;

        #[wasm_bindgen]
        pub async fn load_modules() -> Result<()> {

            let root = {
                let collection = document().get_elements_by_tag_name("head");
                if collection.length() > 0 {
                    collection.item(0).unwrap()
                } else {
                    document().get_elements_by_tag_name("body").item(0).unwrap()
                }
            };
        

            let modules : ModuleMap = MODULES.into_iter().collect();

            for ids in DEPENDENCIES.iter() {
                let futures = ids
                    .iter()
                    .map(|id| {
                        if let Some(module) = modules.get(id) {
                            Some(module.load(&root,&modules))
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
                        Ok(_event) => { },
                        Err(err) => {
                            log_error!("{}", err);
                        }
                    }
                }
            }

            Ok(())
        }
        "###;

        // text += r###"

        // pub async fn load() {
        // "###;

        let mut table = Vec::new();
        for module in collection.modules.iter() {

            let mut imports = Vec::new();
            for import in module.imports.iter() {
                let reference = import
                    .reference()
                    .expect(&format!("failure dereferencing `{}` -> `{}`", import.referrer.display(),import.location));
                
                match import.kind {
                    ReferenceKind::Import => {
                        imports.push(format!("(Import::Detail(\"{}\"),{})",import.what,reference.id));
                    },
                    ReferenceKind::ImportAll => {
                        imports.push(format!("(Import::All,{})",reference.id));
                    },
                    _ => {
                        panic!("wtf");
                    }
                }
            }
            let imports = imports.join(",");
            
            let mut exports = Vec::new();
            for export in module.exports.iter() {
                let reference = export.reference().unwrap();
                exports.push(format!("(\"{}\",{})",export.what,reference.id));
            }
            let exports = exports.join(",");

            // println!("{}",module.ident);            
            // text += &format!("\n\t\t\tconst MODULE_{}: Module = Module {{\n\
            let definition = format!("Module {{\n\
                \turl : Mutex::new(None),\n\
                \tident : \"{}\",\n\
                \tcontent : {},\n\
                \timports : &[{}],\n\
                \texports : &[{}],\n\
            }}", 
                // module.id(&ident_kind),
                module.ident(&ident_kind).to_lowercase(),
                module.ident(&ident_kind),
                imports,
                exports,
            );

            table.push(format!("({},{})",module.id,definition));
        }
        let table_len = table.len();
        let table = table.join(",\n");
        text += &format!("\nconst MODULES : [(ModuleId,Module);{}] = [\n{}\n];\n",table_len, table);

        let mut groups = Vec::new();
        let mut comments = Vec::new();
        for group in collection.groups.iter() {
            let ids = group.iter().map(|m|m.id.to_string()).collect::<Vec<_>>().join(", ");
            let names = group.iter().map(|m|m.ident.clone()).collect::<Vec<_>>().join(", ");
            groups.push(format!("&[{}]",ids));
            comments.push(names)
        }
        let comments = comments.join("\n// ");
        let groups = groups.join(",\n\t");

        text += &format!("\n// {}\n", comments);
        text += &format!("\nconst DEPENDENCIES : &[&[ModuleId]] = &[\n\t{}\n];\n", groups);

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
