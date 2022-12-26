use crate::prelude::*;

#[derive(Debug)]
pub struct Export {
    // pub origin: PathBuf,
    pub referrer: PathBuf,
    pub what: String,
    pub location: String,
    pub reference: Mutex<Option<Arc<FileModule>>>,
}

impl Export {
    pub fn new(referrer: &Path, what: &str, location: &str) -> Export {
        Export {
            referrer: referrer.to_path_buf(),
            what: what.to_string(),
            location: location.to_string(),
            reference: Mutex::new(None),
        }
    }
}
