use crate::prelude::*;

#[derive(Debug)]
pub struct Import {
    // pub origin: PathBuf,
    pub referrer: PathBuf,
    pub what: String,
    pub location: String,
    pub reference: Mutex<Option<Arc<FileModule>>>,
}

impl Import {
    pub fn new(referrer: &Path, what: &str, location: &str) -> Import {
        Import {
            referrer: referrer.to_path_buf(),
            what: what.to_string(),
            location: location.to_string(),
            reference: Mutex::new(None),
        }
    }

    pub fn reference(&self) -> Option<Arc<FileModule>> {
        self.reference.lock().unwrap().as_ref().cloned()
    }

    pub fn warn(&self) {
        log_warn!("Warning","+--- import");
        log_warn!("","| Unable to resolve: `{}`", self.what);
        log_warn!("","| location: `{}`", self.location);
        log_warn!("","| referrer: `{}`", self.referrer.display());
        log_warn!("","+---");
    }
}
