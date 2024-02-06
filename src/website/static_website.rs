use std::collections::HashMap;
use std::fs;
use std::path::Path;

use itertools::Itertools;

use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use crate::website::loaded_file::LoadedFile;
use crate::website::server::Server;

pub struct StaticWebsite {
    resources: HashMap<String, LoadedFile>,
}

impl StaticWebsite {
    pub fn from_assets(directory: &Path) -> anyhow::Result<Self> {
        // First, load public assets
        let public = directory.join("public");
        let dirs = fs::read_dir(public)?;
        let resources = dirs
            .into_iter()
            .map(|dr| {
                dr.map_err(anyhow::Error::from)
                    .map(|d| (d.file_name(), d.path()))
                    .and_then(|(f_n, f_p)| {
                        let resource_str = f_n.to_str().expect("Paths should be parseable");
                        let file = match LoadedFile::from_path(&f_p) {
                            Ok(file) => file,
                            Err(e) => {
                                return Err(e);
                            }
                        };
                        Ok((resource_str.to_string(), file))
                    })
            })
            .try_collect()?;
        Ok(Self { resources })
    }
    pub fn default() -> Self {
        StaticWebsite::from_assets(Path::new("assets/")).expect("There should be an assets folder")
    }
}

fn reload_and_log_if_unable(file: LoadedFile) -> LoadedFile {
    file.try_reload().unwrap_or_else(|(file, err)| {
        println!("Unable to reload due to the following error: {}", err);
        file
    })
}

impl Server for StaticWebsite {
    fn reload(self) -> Self {
        // Check if any of the loaded resources have changed
        let resources: HashMap<String, LoadedFile> = self
            .resources
            .into_iter()
            .map(|(key, file)| (key, reload_and_log_if_unable(file)))
            .collect();
        Self { resources }
    }
    fn serve(&self, http_request: HttpRequest) -> HttpResponse {
        println!("Serving for: {:?}", http_request);
        // Special logic for special uris
        let actual_resource = if http_request.uri == "/" {
            "index.html"
        } else if http_request.uri.starts_with('/') {
            &http_request.uri[1..]
        } else {
            &http_request.uri
        };

        if let Some(resource) = self.resources.get(actual_resource) {
            HttpResponse::from_page(&resource.contents)
        } else {
            self.serve_error(http_request)
        }
    }
}

impl Default for StaticWebsite {
    fn default() -> Self {
        Self::new()
    }
}

impl StaticWebsite {
    pub fn new() -> StaticWebsite {
        StaticWebsite {
            resources: HashMap::default(),
        }
    }
}
