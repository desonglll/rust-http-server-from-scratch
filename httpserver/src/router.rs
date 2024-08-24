use std::io::prelude::*;

use http::httprequest;
use http::httprequest::{HttpRequest, Resource};
use http::httpresponse::HttpResponse;

use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            // If GET request
            httprequest::Method::Get => match &req.resource {
                Resource::Path(s) => {
                    //Parse the URL
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        // if the route begins with /api, invoke Web service
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        // Else, invoke static page handler
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            // If method is not GET request, return 404 page
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}