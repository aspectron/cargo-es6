use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Collection {
    pub idents: HashSet<String>,
    pub groups: Vec<Vec<Arc<Content>>>,
    pub content: Vec<Arc<Content>>,
}

impl Collection {
    pub fn new() -> Collection {
        Collection {
            idents: HashSet::new(),
            content: Vec::new(),
            groups: Vec::new(),
        }
    }
}
