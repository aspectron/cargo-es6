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

impl From<&ReferenceKind> for ContentType {
    fn from(kind: &ReferenceKind) -> Self {
        match kind {
            ReferenceKind::Style => ContentType::Style,
            ReferenceKind::Module => ContentType::Module,
            ReferenceKind::Script => ContentType::Script,
            ReferenceKind::Export => ContentType::Module,
        }
    }
}

// #[derive(Debug)]
// pub struct Referrer {

// }

#[derive(Debug)]
pub struct Reference {
    pub kind: ReferenceKind,
    pub referrer: u64,
    pub what: Option<String>,
    pub location: String,
    pub content: Mutex<Option<Arc<Content>>>,
}

impl Reference {
    pub fn new(
        kind: ReferenceKind,
        referrer: u64,
        what: Option<&str>,
        location: &str,
    ) -> Reference {
        let what = what.map(|s| s.trim().to_string());

        Reference {
            kind,
            referrer,
            what,
            location: location.trim().to_string(),
            content: Mutex::new(None),
        }
    }

    pub fn content(&self) -> Option<Arc<Content>> {
        self.content.lock().unwrap().as_ref().cloned()
    }

    pub fn warn(&self, db: &Db) {
        log_warn!("Warning", "+--- Unable to resolve");
        if let Some(what) = &self.what {
            log_warn!("", "| what: `{:?}`", what);
        }
        log_warn!("", "| location: `{}`", self.location);
        let referrer = db
            .get_file(&self.referrer)
            .unwrap_or_else(|| panic!("unable to get file id 0x{:16x}", self.referrer));
        log_warn!("", "| referrer: `{}`", referrer.location.display());
        log_warn!("", "+---");
    }

    pub fn error(&self, db: &Db) {
        log_error!("+--- Unable to resolve");
        log_error!("| location: `{}`", self.location);
        let referrer = db
            .get_file(&self.referrer)
            .unwrap_or_else(|| panic!("unable to get file id 0x{:16x}", self.referrer));
        log_warn!("", "| referrer: `{}`", referrer.location.display());
        log_error!("+---");
    }
}
