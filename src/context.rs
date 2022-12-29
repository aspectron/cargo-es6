use crate::prelude::*;

#[derive(Debug)]
pub struct Options {
}

impl Default for Options {
    fn default() -> Self {
        Options {
        }
    }
}

#[derive(Debug)]
pub struct Context {

    pub manifest : Manifest,
    // pub target_file : PathBuf,
    pub target_folder : PathBuf,
    pub target_folder_src : PathBuf,
    pub project_file : PathBuf,
    pub project_folder : PathBuf,
    pub node_modules : PathBuf,
    pub ignore : Filter,
}

impl Context {
    pub async fn create(
        location : Option<String>,
        // output : Option<String>,
        _options: Options,
    ) -> Result<Context> {

        let manifest_toml = Manifest::locate(location).await?;
        log_info!("Manifest","`{}`",manifest_toml.to_str().unwrap());
        let manifest = Manifest::load(&manifest_toml).await?;
        let manifest_folder = manifest_toml.parent().unwrap().to_path_buf();

        let project_file = manifest_folder.join(&manifest.settings.project).canonicalize().await?;
        let project_folder = project_file.parent().unwrap().to_path_buf();
        let node_modules = project_folder.join("node_modules");
        let target_folder = manifest_folder.join(&manifest.settings.target);//.canonicalize().await?;
        let target_folder_src = target_folder.join("src");//.canonicalize().await?;
        // let target_file = manifest_folder.join(&manifest.settings.target);//.canonicalize().await?;
        // let target_folder = target_file.parent().unwrap().canonicalize().await?;//to_path_buf();
        log_info!("Project","`{}`",project_folder.to_str().unwrap());
        log_info!("Target","`{}`",target_folder.to_str().unwrap());

        let ignore = if let Some(ignore) = &manifest.settings.ignore {
            Filter::new(&ignore.iter().map(|s|s.as_str()).collect::<Vec<_>>())
        } else {
            Filter::default()
        };

        let ctx = Context {
            manifest,
            // target_file,
            target_folder,
            target_folder_src,
            project_file,
            project_folder,
            node_modules,
            ignore,
        };

        Ok(ctx)
    }

    pub async fn ensure_folders(&self) -> Result<()> {
        let folders = [
            &self.target_folder_src,
        ];
        for folder in folders {
            if !std::path::Path::new(folder).exists() {
                std::fs::create_dir_all(folder)?;
            }
        }

        Ok(())
    }

    pub async fn clean(&self) -> Result<()> {
        // if self.target_folder_src.exists().await {
        //     // log_info!("Cleaning","`{}`",self.target_folder.display());
        //     async_std::fs::remove_dir_all(&self.target_folder_src).await?;
        // }
        Ok(())
    }

}

