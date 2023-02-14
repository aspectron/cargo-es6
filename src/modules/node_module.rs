use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageJson {
    pub version: Option<String>,
    pub name: Option<String>,
    pub exports: Option<HashMap<String, serde_json::Value>>,
    pub dependencies: Option<HashMap<String, String>>,
    pub browser: Option<serde_json::Value>,
    pub files: Option<Vec<String>>,
    pub module: Option<String>,
    pub main: Option<String>,
}

#[derive(Debug)]
pub struct NodeModule {
    pub id: u64,
    pub is_project: bool,
    pub version: Option<String>,
    pub name: String,
    pub alt_name: String,
    pub default: Option<Arc<Content>>,
    pub exports: HashMap<PathBuf, PathBuf>,
    pub files: Vec<Arc<Content>>,
    pub package_json: PackageJson,
    pub module_folder: PathBuf,
    pub base_folder: PathBuf,
}

impl NodeModule {
    pub fn load(db: &mut Db, module_folder: &Path) -> Result<NodeModule> {
        // println!("loading {}", module_folder.display());
        let text = std::fs::read_to_string(&module_folder.join("package.json"))?;
        let package_json: PackageJson = serde_json::from_str(&text)?;

        let (name, alt_name) =
            Self::get_alt_name(package_json.name.as_ref().unwrap_or_else(|| {
                panic!("`package.json` missing name property: {package_json:#?}")
            }));

        let id = name.u64_hash();
        let base_folder = module_folder
            .strip_prefix(&db.ctx.project_folder)?
            .to_path_buf();

        // build export tables
        let mut exports = HashMap::new();
        if let Some(package_json_exports) = &package_json.exports {
            for (k, v) in package_json_exports.iter() {
                if v.is_string() {
                    let k = module_folder.join(k).parse_dot()?.to_path_buf();
                    let v = module_folder
                        .join(v.as_str().unwrap())
                        .parse_dot()?
                        .to_path_buf();
                    exports.insert(k, v);
                } else if v.is_object() {
                    let map = v.as_object().unwrap();

                    let path = if let Some(path) = map.get("import") {
                        path
                    } else if let Some(path) = map.get("default") {
                        path
                    } else {
                        return Err(format!("error parsing export `{k}` in `{name}`").into());
                    };

                    if let Some(path) = path.as_str() {
                        let k = module_folder.join(k).parse_dot()?.to_path_buf();
                        let v = module_folder.join(path).parse_dot()?.to_path_buf();
                        // println!("{} -> {}", k.display(), v.display());
                        exports.insert(k, v);
                    } else {
                        return Err(format!(
                            "error parsing export `{k}` in `{name}` (must be a string)"
                        )
                        .into());
                    }
                }
            }
        }

        // add `browser` property to export tables
        if let Some(browser) = &package_json.browser {
            if browser.is_string() {
                let k = module_folder.join("index.js");
                let v = PathBuf::from(browser.to_string());
                exports.insert(k, v);
            } else if browser.is_object() {
                let browser = browser.as_object().unwrap();
                for (k, v) in browser.iter() {
                    if !v.is_string() {
                        continue;
                    }
                    let k = module_folder.join(k).parse_dot()?.to_path_buf();
                    let v = module_folder
                        .join(v.as_str().unwrap())
                        .parse_dot()?
                        .to_path_buf();

                    exports.insert(k, v);
                }
            } else {
                return Err(format!("error parsing `browser` option in `{name}`").into());
            }
        }

        let node_module = NodeModule {
            id,
            is_project: false,
            default: None,
            version: package_json.version.clone(),
            name,
            alt_name,
            exports, //: HashMap::new(),
            files: Vec::new(),
            package_json,
            base_folder,
            module_folder: module_folder.to_path_buf(),
        };

        Ok(node_module)
    }

    // TODO...

    pub fn resolve_absolute_content(&self, db: &Db, location: PathBuf) -> Result<PathBuf> {
        let absolute_path_reference = self.get_absolute_path(db).join(&location);
        let absolute_path = self.resolve_exports_absolute(absolute_path_reference);

        if !absolute_path.is_file() {
            // println!("absolute: {}", absolute_path_reference.display());
            // println!("absolute: {}", absolute_path.display());
            // println!("exports: {:#?}", self.exports);
            return Err(format!(
                "Unable to find `{}` in `{}` at `{}`",
                location.display(),
                self.name,
                self.module_folder.display(),
            )
            .into());
        } else {
            Ok(absolute_path)
        }
    }

    pub fn resolve_exports_absolute(&self, mut location: PathBuf) -> PathBuf {
        while let Some(l) = self.exports.get(&location) {
            location = l.clone();
        }

        location
    }

    pub fn main_file_absolute(&self, _ctx: &Context) -> Result<Option<PathBuf>> {
        let mut main_file = self
            .package_json
            .module
            .clone()
            .or(self.package_json.main.clone())
            .unwrap_or("index.js".to_string());
        if main_file.is_empty() {
            main_file = "index.js".to_string();
        }

        let main_file_absolute = self
            .module_folder
            .join(main_file)
            .parse_dot()?
            .to_path_buf();
        // println!("dereferencing main_file: {}", main_file_absolute.display());
        let main_file_absolute = self.resolve_exports_absolute(main_file_absolute);
        // println!("dereferenced main_file: {}", main_file_absolute.display());
        if main_file_absolute.exists() {
            Ok(Some(main_file_absolute))
        } else {
            Ok(None)
        }
    }

    pub fn main_file_relative(&self, ctx: &Context) -> Result<Option<PathBuf>> {
        if let Some(main_file_absolute) = self.main_file_absolute(ctx)? {
            let main_file = main_file_absolute
                .strip_prefix(&self.module_folder)?
                .to_path_buf();
            Ok(Some(main_file))
        } else {
            Ok(None)
        }
    }

    pub fn base_folder(&self) -> &PathBuf {
        &self.base_folder
    }

    pub fn get_absolute_path(&self, db: &Db) -> PathBuf {
        db.ctx.project_folder.join(&self.base_folder)
    }

    pub fn dependencies(&self) -> Option<Vec<&String>> {
        if let Some(dependencies) = &self.package_json.dependencies {
            let dependencies = dependencies.keys().collect::<Vec<_>>();
            if dependencies.is_empty() {
                None
            } else {
                Some(dependencies)
            }
        } else {
            None
        }
    }

    pub fn get_alt_name(full_name: &str) -> (String, String) {
        if full_name.starts_with('@') {
            let alt_name = full_name
                .split('/')
                .collect::<Vec<_>>()
                .get(1)
                .unwrap()
                .to_string();
            (full_name.to_string(), alt_name)
        } else {
            (full_name.to_string(), full_name.to_string())
        }
    }

    pub fn get_name_from_location(location: &str) -> String {
        if location.starts_with('@') {
            let components = Path::new(location)
                .components()
                .into_iter()
                .map(|c| c.as_os_str().to_str().unwrap())
                .collect::<Vec<_>>();
            let mut node_module = Vec::new();
            for n in 0..std::cmp::min(2, components.len()) {
                node_module.push(components[n].to_string());
            }
            node_module.join("/")
        } else {
            let components = Path::new(location)
                .components()
                .into_iter()
                .map(|c| c.as_os_str().to_str().unwrap())
                .collect::<Vec<_>>();
            components[0].to_string()
        }
    }
}

impl ToString for NodeModule {
    fn to_string(&self) -> String {
        let fields = [
            // ("name", Some(self.name.clone())),
            ("name", Some(self.name.clone())),
            ("version", self.version.clone()),
            (
                "default",
                self.default
                    .as_ref()
                    .map(|content| u64_to_hex_str(&content.id)),
            ),
        ]
        .iter()
        .filter(|(_k, v)| v.is_some())
        .map(|(k, v)| format!("{} = \"{}\"", k, v.as_ref().unwrap()))
        .collect::<Vec<_>>()
        .join(", ");
        format!("{{ {fields} }}")
    }
}
