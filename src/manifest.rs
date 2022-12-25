use async_std::fs::*;
use async_std::path::{PathBuf, Path};
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub settings : Settings,
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
            &location.join("es6rs.toml")
        ];

        for location in locations.iter() {
            match location.canonicalize().await {
                Ok(location) => {
                    if location.is_file().await {
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
                return Err(format!("Error loading nw.toml: {}", err).into());
            }

        };

        Ok(manifest)
    }
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub project : String,
    pub target : String,
    pub ignore : Option<Vec<String>>,
}