use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub settings: Settings,
    pub manifest: Option<HashMap<String, HashMap<String, HashMap<String, toml::Value>>>>,
}

impl Manifest {
    pub fn locate(location: Option<String>) -> Result<PathBuf> {
        let cwd = current_dir();

        let location = if let Some(location) = location {
            if location.starts_with("~/") {
                home::home_dir()
                    .expect("unable to get home directory")
                    .join(&location[2..])
                    .into()
            } else {
                let location = Path::new(&location).to_path_buf();
                if location.is_absolute() {
                    location
                } else {
                    cwd.join(&location)
                }
            }
        } else {
            cwd
        };

        let locations = [
            &location,
            &location.with_extension("toml"),
            &location.join("es6.toml"),
        ];

        for location in locations.iter() {
            match location.canonicalize() {
                Ok(location) => {
                    if location.is_file() {
                        return Ok(location);
                    }
                }
                _ => {}
            }
        }

        Err(format!("Unable to locate 'wahoo.toml' manifest").into())
    }

    pub fn load(toml_file: &PathBuf) -> Result<Manifest> {
        let toml_text = std::fs::read_to_string(toml_file)?;
        let manifest: Manifest = match toml::from_str(&toml_text) {
            Ok(manifest) => manifest,
            Err(err) => {
                return Err(format!("Error loading es6.toml: {}", err).into());
            }
        };
        // println!("{:#?}",manifest.manifest);
        Ok(manifest)
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "lowercase")]
// pub enum RustOutput {
//     Lib,
//     Mod,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Module,
    Script,
}

impl Into<ContentType> for ProjectType {
    fn into(self) -> ContentType {
        match self {
            ProjectType::Module => ContentType::Module,
            ProjectType::Script => ContentType::Script,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub name: String,
    pub project: String,
    pub module: Option<String>,
    #[serde(rename = "type")]
    pub project_type: ProjectType,
    // pub generate : RustOutput,
    pub target: String,
    pub ignore: Option<Vec<String>>,
    pub wasm: Option<Wasm>,
    pub verbose: Option<bool>,
    pub enums: Option<Enums>,
    pub replace: Option<Vec<Replace>>,
    // TODO
    pub preload: Option<Vec<String>>,
    // pub optimize : Option<bool>,
    pub include_all_exports: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wasm {
    pub name: String,
    pub folder: String,
    pub outdir: String,
    pub dev: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enums {
    pub exports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Replace {
    Location {
        regex: Option<String>,
        from: Option<String>,
        to: String,
    },
}
