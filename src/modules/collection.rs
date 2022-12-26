use crate::prelude::*;

pub struct Collection {
    pub idents : HashSet<String>,
    pub modules : Vec<Arc<FileModule>>,
}

impl Collection {
    pub fn new() -> Collection {
        Collection {
            idents : HashSet::new(),
            modules : Vec::new(),
        }
    }
}
