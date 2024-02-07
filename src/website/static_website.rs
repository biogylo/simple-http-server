use std::collections::HashMap;
use std::fs;
use std::path::Path;

use itertools::Itertools;

use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use crate::website::loaded_file::{LoadedFile, ReloadResult};
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

impl Server for StaticWebsite {
    fn serve(&mut self, http_request: &HttpRequest) -> HttpResponse {
        // Special logic for special uris
        let actual_resource = if http_request.uri == "/" {
            "index.html"
        } else if http_request.uri.starts_with('/') {
            &http_request.uri[1..]
        } else {
            &http_request.uri
        };
        let Some(resource) = self.resources.get(actual_resource) else {
            return self.redirect_to_index(http_request);
        };
        let reload_result = resource.try_reload();
        let file_ref = match reload_result {
            ReloadResult::NotNeeded(file_ref) => file_ref,
            ReloadResult::Reloaded(file) => {
                self.resources.insert(actual_resource.to_string(), file);
                &self.resources[actual_resource]
            }
            ReloadResult::ErrorDidntReload((file_ref, err_string)) => {
                log::error!("Unable to reload resource! {}", err_string);
                file_ref
            }
        };
        HttpResponse::from_page(&file_ref.contents)
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
