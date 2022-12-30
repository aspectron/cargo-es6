use crate::prelude::*;

#[derive(Debug)]
pub enum ReferenceKind {
    Style,
    Module,
    Script,
    Export,
}

impl ToString for ReferenceKind {
    fn to_string(&self) -> String {
        match self {
            ReferenceKind::Style => "stylesheet".to_string(),
            ReferenceKind::Module => "module-import".to_string(),
            ReferenceKind::Script => "script-import".to_string(),
            // ReferenceKind::ImportAll => "import-all".to_string(),
            ReferenceKind::Export => "export".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Reference {
    pub kind : ReferenceKind,
    pub referrer: PathBuf,
    pub what: Option<String>,
    pub location: String,
    // pub component : String,
    pub reference: Mutex<Option<Arc<Content>>>,
}

impl Reference {
    pub fn new(kind: ReferenceKind, referrer: &Path, what: Option<&str>, location: &str) -> Reference {


        // .from_case(Case::Kebab).to_case(Case::Title)

        let what = what.map(|s| s.trim().to_string());

        Reference {
            kind,
            referrer: referrer.to_path_buf(),
            what,
            location: location.trim().to_string(),
            // component,
            reference: Mutex::new(None),
        }
    }

    pub fn content(&self) -> Option<Arc<Content>> {
        self.reference.lock().unwrap().as_ref().cloned()
    }

    pub fn warn(&self) {
        log_warn!("Warning","+--- Unable to resolve");
        if let Some(what) = &self.what {
            log_warn!("","| what: `{:?}`", what);
        }
        log_warn!("","| location: `{}`", self.location);
        log_warn!("","| referrer: `{}`", self.referrer.display());
        log_warn!("","+---");
    }

    pub fn error(&self) {
        log_error!("+--- Unable to resolve");
        log_error!("| location: `{}`", self.location);
        log_error!("| referrer: `{}`", self.referrer.display());
        log_error!("+---");
    }



}
