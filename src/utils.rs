use crate::prelude::*;
use regex::Regex;
use std::borrow::Cow;

pub fn search_upwards<P>(folder: P, filename: &str) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let mut folder = folder.as_ref();

    loop {
        let file_path = folder.join(filename);
        if file_path.is_file() {
            return Some(file_path);
        }

        if let Some(parent) = folder.parent() {
            folder = parent;
        } else {
            return None;
        }
    }
}

pub fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

// pub async fn find_file(folder: &Path,files: &[&str]) -> Result<PathBuf> {
pub fn find_file(folder: &Path, files: &[String]) -> Result<PathBuf> {
    for file in files {
        let path = folder.join(file);
        if let Ok(path) = path.canonicalize() {
            if path.is_file() {
                return Ok(path);
            }
        }
    }
    return Err(format!(
        "Unable to locate any of the files: {} \nfrom {:?} directory",
        files.join(", "),
        folder.to_str().unwrap_or("")
    )
    .into());
}

pub fn get_env_defs(strings: &Vec<String>) -> Result<Vec<(String, String)>> {
    let regex = Regex::new(r"([^=]+?)=(.+)").unwrap();

    let mut parsed_strings = Vec::new();

    for string in strings {
        let captures = regex.captures(string).unwrap();
        if captures.len() != 2 {
            return Err(format!("Error parsing the environment string: '{string}'").into());
        }
        let a = captures[1].to_string();
        let b = captures[2].to_string();

        parsed_strings.push((a, b));
    }

    Ok(parsed_strings)
}

pub fn is_hidden<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    let is_hidden = path
        .as_ref()
        .components()
        .any(|f| f.as_os_str().to_string_lossy().starts_with('.'));

    is_hidden
}

pub fn get_files<P>(
    folder: P,
    aggregator: Option<&Filter>,
    filter: Option<&Filter>,
) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    // let path =

    let list = WalkDir::new(folder.as_ref())
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let relative = path
                .strip_prefix(folder.as_ref())
                .expect("get_files() failure");
            let relative_str = relative.to_string_lossy();

            if aggregator
                .map(|f| f.is_match(&relative_str))
                .and(filter.map(|f| f.is_match(&relative_str)))
                .unwrap_or(false)
                && path.is_file()
            {
                Some(Path::new(relative).to_path_buf())
            } else {
                None
            }
        });

    Ok(list.collect())
}

pub trait NormalizePath {
    fn normalize(&self) -> Result<Cow<Path>>;
}

impl NormalizePath for Path {
    fn normalize(&self) -> Result<Cow<Path>> {
        Ok(self.parse_dot()?)
    }
}

// pub fn normalize<P : AsRef<Path>>(p: &P) -> Result<Cow<Path>> {
//     Ok(p.as_ref().parse_dot()?)
// }
