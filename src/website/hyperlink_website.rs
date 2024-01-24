use std::ffi::OsString;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

use itertools::Itertools;

use crate::handler::static_page::StaticPageHandler;
use crate::website::static_website::StaticWebsite;

fn dir_entry_is_html(dir_entry: &DirEntry) -> bool {
    dir_entry.path().extension().is_some_and(|e| e == "html")
}

fn get_resource_path_and_resource_name(dir_entry: DirEntry) -> (PathBuf, OsString) {
    let resource_path = dir_entry.path();
    let path = dir_entry.path();
    let path_str = path
        .file_name()
        .expect("Gotta be")
        .to_str()
        .expect("We are assuming this is good");
    let (resource_name, _) = path_str.split_once(".").unwrap();
    let resource_name: String = "/".to_owned() + resource_name;
    (resource_path, resource_name.into())
}

pub fn build_hyperlinked_website<'a, 'b>(directory: PathBuf) -> anyhow::Result<StaticWebsite> {
    // Gotta do the following
    // 1. List all jinja2 files in the directory
    let dirs: Vec<DirEntry> = fs::read_dir(directory)?.try_collect()?;

    let mut server = StaticWebsite::default();

    for (resource_path, resource_name) in dirs
        .into_iter()
        .filter(dir_entry_is_html)
        .map(get_resource_path_and_resource_name)
    {
        let page_contents = fs::read_to_string(resource_path).map_err(anyhow::Error::from)?;
        let handler = StaticPageHandler::new(page_contents);
        server = server.with_endpoint(resource_name, Box::new(handler))
    }
    Ok(server)

    // TODO: Make this a const fn
}
