mod file_explorer;

use anyhow::Result;
use hyper::{Body, Request, Response};
use std::{convert::TryFrom, sync::Arc};

use crate::Config;

use super::middleware::Middleware;

use self::file_explorer::{make_file_explorer_handler, FileExplorer};

#[derive(Clone)]
pub struct HttpHandler {
    file_explorer: Arc<FileExplorer>,
    middleware: Arc<Middleware>,
}

impl HttpHandler {
    pub async fn handle_request(self, request: Request<Body>) -> Result<Response<Body>> {
        let handler = make_file_explorer_handler(self.file_explorer);
        let middleware = Arc::clone(&self.middleware);
        let response = middleware.handle(request, handler).await;

        Ok(response)
    }
}

impl From<Config> for HttpHandler {
    fn from(config: Config) -> Self {
        let file_explorer = Arc::new(FileExplorer::new(config.root_dir()));
        let middleware = Middleware::try_from(config).unwrap();
        let middleware = Arc::new(middleware);

        HttpHandler {
            file_explorer,
            middleware,
        }
    }
}