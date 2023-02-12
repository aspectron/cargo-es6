// // https://crates.io/api/v1/crates/name/version/download

// use crate::prelude::*;


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ExternalContentReference {
//     pub node_module : u64,
//     pub location : String,
// }

// impl TryFrom<(&Db,&Arc<Content>)> for ExternalContentReference {
//     type Error = Error;
//     fn try_from((db,content): (&Db,&Arc<Content>)) -> Result<ExternalContentReference> {
//         // println!("{:?}", content);
//         // let base = content.base_folder
//         //     .strip_prefix(&ctx.project_folder)
//         //     .map_err(|err|format!("{}: {} in {}",err,ctx.project_folder.display(),content.base_folder.display()))?
//         //     .to_str()
//         //     .unwrap()
//         //     .to_string();
//         // println!("base_folder: {}",content.base_folder.display());
//         // println!("absolute: {}",content.absolute.display());
//         let base = content.node_module;

//         let base_folder = match base {
//             Some(id) => {
//                 db.node_modules_by_id
//                     .get(&id)
//                     .unwrap()
//                     .folder(&db.ctx)
//                     .clone()
//             },
//             None => {
//                 db.ctx.project_folder.clone()
//             }
//         };

//         let location = content.location
//             .strip_prefix(&base_folder)
//             .map_err(|err|format!("{}: {} in {}",err,content.location.display(),base_folder.display()))?
//             .to_str()
//             .unwrap()
//             .to_string();

//         Ok(ExternalContentReference {
//             node_module: base,
//             location,
//         })
//     }
// }

// impl ToString for ExternalContentReference {
//     fn to_string(&self) -> String {

//         let fields = [
//             ("node_module", self.node_module.map(|id|u64_to_hex_str(&id))),
//             ("location", Some(self.location.clone())),
//         ]
//         .iter()
//         .filter(|(k,v)|v.is_some())
//         .map(|(k,v)|format!("{} = \"{}\"",k,v.as_ref().unwrap()))
//         .collect::<Vec<_>>()
//         .join(", ");

//         format!("{{ {} }}", fields)
//     }
// }

