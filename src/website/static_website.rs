use std::collections::HashMap;
use std::path::PathBuf;

use relative_path::RelativePath;

use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use crate::website::loaded_file::{LoadedFile, ReloadResult};
use crate::website::server::Server;

pub struct StaticWebsite {
    public_directory: PathBuf,
    cache: HashMap<String, LoadedFile>,
}

impl StaticWebsite {
    pub fn new(public_directory: PathBuf) -> Self {
        StaticWebsite {
            public_directory,
            cache: Default::default(),
        }
    }
}

impl Server for StaticWebsite {
    fn serve(&mut self, http_request: &HttpRequest) -> HttpResponse {
        // Special logic for special uris
        let key = http_request.uri.to_string();

        match self.cache.get(&key) {
            None => {
                // Gotta see if it exists, and put into cache
                let relative_path = RelativePath::new(&key);
                let resource_path = relative_path.to_path(&self.public_directory);
                if !resource_path.is_file() {
                    return self.redirect_to_index(http_request);
                }
                let Ok(loaded_file) = LoadedFile::from_path(&resource_path) else {
                    return self.redirect_to_index(http_request);
                };
                self.cache.insert(key.clone(), loaded_file);
                HttpResponse::from_page(&self.cache[&key].contents)
            }
            Some(resource) => {
                // Gotta see if it needs reloading and do whats needed
                if !resource.filepath.is_file() {
                    return self.redirect_to_index(http_request);
                }
                let reload_result = resource.try_reload();

                let file_ref = match reload_result {
                    ReloadResult::NotNeeded(file_ref) => file_ref,
                    ReloadResult::Reloaded(file) => {
                        self.cache.insert(key.clone(), file);
                        &self.cache[&key]
                    }
                    ReloadResult::ErrorDidntReload((file_ref, err_string)) => {
                        log::error!("Unable to reload resource! {}", err_string);
                        file_ref
                    }
                };
                HttpResponse::from_page(&file_ref.contents)
            }
        }
    }
}
