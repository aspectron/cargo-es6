use crate::prelude::*;

pub struct Collection {
    pub idents : HashSet<String>,
    pub groups : Vec<Vec<Arc<FileModule>>>,
    pub modules : Vec<Arc<FileModule>>,
}

impl Collection {
    pub fn new() -> Collection {
        Collection {
            idents : HashSet::new(),
            modules : Vec::new(),
            groups : Vec::new(),
        }
    }

    // pub fn insert(&mut self, module : &Arc<FileModule>) -> bool{
    //     let ident = module.ident.clone();
    //     if self.idents.contains(&ident) {
    //         return false;
    //     }

    //     self.idents.insert(ident);
    // }
}
