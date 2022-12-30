use async_std::fs::*;
use crate::prelude::*;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Dependency {
//     pub dependencies : Vec<String>,
//     pub settings : Vec<String>,
//     pub dependencies : Vec<String>,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub settings : Settings,
    pub manifest : HashMap<String,HashMap<String,String>>,
}

impl Manifest {
    pub async fn locate(location: Option<String>) -> Result<PathBuf> {
        let cwd = current_dir().await;

        let location = if let Some(location) = location {
            if location.starts_with("~/") {
                home::home_dir().expect("unable to get home directory").join(&location[2..]).into()
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
            &location.join("es6.toml")
        ];

        for location in locations.iter() {
            match location.canonicalize() {
                Ok(location) => {
                    if location.is_file() {
                        return Ok(location)
                    }
                }, 
                _ => { }
            }
        }

        Err(format!("Unable to locate 'wahoo.toml' manifest").into())
    }
    
    pub async fn load(toml_file : &PathBuf) -> Result<Manifest> {
        let toml_text = read_to_string(toml_file).await?;
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
    pub project : String,
    #[serde(rename = "type")]
    pub project_type : ProjectType,
    // pub generate : RustOutput,
    pub target : String,
    pub ignore : Option<Vec<String>>,
    pub wasm : Option<Wasm>,
    pub verbose : Option<bool>,
    pub enums : Option<Enums>,
    pub replace : Option<Vec<Replace>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wasm {
    pub name : String,
    pub folder : String,
    pub outdir : String,
    pub dev : Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enums {
    pub exports : Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Replace {
    Location {
        regex : Option<String>,
        from : Option<String>,
        to : String,
    }
}
