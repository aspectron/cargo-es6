use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub default : String,
    pub development : Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageJson {
    pub name : String,
    pub exports : Option<HashMap<String, Export>>,
    pub dependencies : Option<HashMap<String, String>>,
    pub files : Option<Vec<String>>,
    pub module : Option<String>,
    pub main : Option<String>,
}

#[derive(Debug)]
pub struct NodeModule {
    pub folder : PathBuf,
    pub absolute : PathBuf,
    pub name : String,
    pub full_name : Option<String>,
    pub root : Option<Arc<FileModule>>,
    pub exports : HashMap<String, Arc<FileModule>>,
    pub files : Vec<Arc<FileModule>>,
}

impl NodeModule {
    pub async fn load<P>(ctx: &Context, absolute: P, package_json: PackageJson) -> Result<NodeModule> 
    where P: AsRef<Path> {

        // let files = Vec::new();
        let absolute = absolute.as_ref();
        let folder = absolute.parent().unwrap();

        let exports = if let Some(node_module_exports) = package_json.exports {
            let mut exports = HashMap::new();
            for (file, export) in node_module_exports.iter() {
                let path = folder.join(&export.default).canonicalize().await?;
                let module = FileModule::load(ctx,folder, &path).await?;
                exports.insert(file.to_string(), Arc::new(module));
                if let Some(development) = &export.development {
                    let path = folder.join(development).canonicalize().await?;
                    let module = FileModule::load(ctx,folder, &path).await?;
                    exports.insert(file.to_string(), Arc::new(module));
                }
            }
            exports
        } else {
            HashMap::new()
        };

        let filter = Filter::new(&["*.{js,mjs}"]);
        let relative_files = if let Some(files) = package_json.files {

            let globs = files
                .iter()
                .map(|s|s.as_str())
                .collect::<Vec<_>>();
            // globs.push("*.js");
            // println!("globs: {:#?}", globs);
            let aggregator = Filter::new(&globs);
            let list = get_files(&folder, Some(&aggregator),Some(&filter)).await?;
            // println!("{:#?}",list);
            list
        } else {
            let aggregator = Filter::new(&["*.{js,mjs}"]);
            get_files(&folder, Some(&aggregator),None).await?
        };


        let mut files = Vec::new();
        for relative in relative_files.iter() {
            files.push(Arc::new(FileModule::load(ctx,folder,relative).await?));
        } 

        let mut root_file = package_json
            .module
            .clone()
            .or(package_json.main.clone())
            .unwrap_or("index.js".to_string());
        if root_file.len() == 0 {
            root_file = "index.js".to_string();
        }

        let root = match folder.join(Path::new(&root_file)).canonicalize().await {
            Ok(path) => {
                let relative = path.strip_prefix(folder).unwrap().to_path_buf();
                let m = Arc::new(FileModule::load(ctx,folder,&relative).await?);
                files.push(m.clone());
                Some(m)
            },
            Err(_) => {
                log_warn!("Warning","node module `{}` contains no root file",absolute.display());
                None
            }
        };

        let (name, full_name) = if package_json.name.starts_with("@") {
            let name = package_json.name
                .split("/")
                .collect::<Vec<_>>()
                .get(1)
                .unwrap()
                .to_string();
            (name, Some(package_json.name.clone()))
        } else {
            (package_json.name.clone(), None)
        };

        let node_module = NodeModule {
            folder : folder.to_path_buf(),
            absolute : absolute.to_path_buf(),
            name,// : package_json.name.clone(),
            full_name,// : package_json.name.clone(),
            exports,
            files,
            root,
        };

        Ok(node_module)
    }

}