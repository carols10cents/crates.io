use std::error::Error;

use conduit::{Request, Response, Handler};
use conduit_static::Static;
use conduit_middleware::AroundMiddleware;

use util::RequestProxy;

pub struct Middleware {
    handler: Option<Box<Handler>>,
    dist: Static,
}

impl Middleware {
    pub fn new() -> Middleware {
        Middleware {
            handler: None,
            dist: Static::new("dist"),
        }
    }
}

impl AroundMiddleware for Middleware {
    fn with_handler(&mut self, handler: Box<Handler>) {
        self.handler = Some(handler);
    }
}

impl Handler for Middleware {
    fn call(&self, req: &mut Request) -> Result<Response, Box<Error+Send>> {
        // First, attempt to serve a static file. If we're missing a static
        // file, then keep going.
        match self.dist.call(req) {
            Ok(ref resp) if resp.status.0 == 404 => {}
            ret => return ret,
        }

        // Second, if we're requesting html, then we've only got one page so
        // serve up that page. Otherwise proxy on to the rest of the app.
        let wants_html = {
            let content = req.headers().find("Accept").unwrap_or(Vec::new());
            content.iter().any(|s| s.contains("html"))
        };
        if wants_html {
            self.dist.call(&mut RequestProxy {
                other: req,
                path: Some("/index.html"),
                method: None,
            })
        } else {
            self.handler.as_ref().unwrap().call(req)
        }
    }
}
