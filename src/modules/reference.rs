use crate::prelude::*;

#[derive(Debug)]
pub enum ReferenceKind {
    Import,
    ImportAll,
    Export,
}

impl ToString for ReferenceKind {
    fn to_string(&self) -> String {
        match self {
            ReferenceKind::Import => "import".to_string(),
            ReferenceKind::ImportAll => "import-all".to_string(),
            ReferenceKind::Export => "export".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Reference {
    pub kind : ReferenceKind,
    pub referrer: PathBuf,
    pub what: String,
    pub location: String,
    pub reference: Mutex<Option<Arc<FileModule>>>,
}

impl Reference {
    pub fn new(kind: ReferenceKind, referrer: &Path, what: &str, location: &str) -> Reference {
        Reference {
            kind,
            referrer: referrer.to_path_buf(),
            what: what.trim().to_string(),
            location: location.trim().to_string(),
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
