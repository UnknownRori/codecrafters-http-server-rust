use std::sync::Arc;

use super::{FnController, HttpMethod, Request, ResponseBuilder};

pub struct Route<T> {
    url: Vec<String>,
    controller: FnController<T>,
}

unsafe impl<T> Send for Route<T> {}
unsafe impl<T> Sync for Route<T> {}

impl<T> Route<T> {
    fn new(url: Vec<String>, controller: FnController<T>) -> Self {
        Route { url, controller }
    }

    pub fn controller(&self) -> &FnController<T> {
        &self.controller
    }
}

pub struct Routes<T> {
    get: Vec<Arc<Route<T>>>,
    post: Vec<Arc<Route<T>>>,
    not_found_route: Option<Arc<Route<T>>>,
}

unsafe impl<T> Send for Routes<T> {}
unsafe impl<T> Sync for Routes<T> {}

impl<T> Routes<T> {
    pub fn new() -> Self {
        Self {
            get: Vec::new(),
            post: Vec::new(),
            not_found_route: None,
        }
    }

    /// Resolve URL using Dynamic URL Matching
    #[allow(unreachable_code)]
    pub fn resolve(&self, request: &mut Request) -> Option<Arc<Route<T>>> {
        println!(
            "Method : {:#?}\npath: : {}\n\n",
            request.method(),
            request.path()
        );

        let routes: &Vec<Arc<Route<T>>> = match request.method() {
            HttpMethod::Get => self.get.as_ref(),
            HttpMethod::Post => self.post.as_ref(),
        };

        {
            let path: Vec<_> = request.path().split("/").collect();
            for route in routes.iter().filter(|r| r.url.len() == path.len()) {
                let mut is_matched = false;
                let mut dynamic_param = vec![];
                // println!("Routes : {}", route.url.join("/"));

                for (path_part, route_part) in path.iter().zip(route.url.iter()) {
                    // println!("{:#?} | {:#?}", path_part, route_part);
                    if route_part.starts_with('{') && route_part.ends_with('}') {
                        dynamic_param.push(path_part.to_string());
                    } else if path_part != route_part {
                        is_matched = false;
                        break;
                    } else {
                        is_matched = true;
                    }
                }

                // println!("");

                if is_matched {
                    request.set_param(dynamic_param);
                    return Some(Arc::clone(&route));
                }
            }

            None
        }
    }

    pub fn set_not_found_route(&mut self, controller: FnController<T>) {
        let route = Arc::new(Route::new(vec![], controller));
        self.not_found_route = Some(route);
    }

    pub fn get_not_found_route(&self) -> Arc<Route<T>> {
        Arc::clone(
            &(&self.not_found_route)
                .clone()
                .unwrap_or(Arc::new(Route::new(
                    vec![],
                    Arc::new(|_, _| {
                        let response = ResponseBuilder::new().code(super::HttpCode::Err404);
                        Ok(response.into())
                    }),
                ))),
        )
    }

    pub fn register_route(&mut self, method: HttpMethod, url: String, controller: FnController<T>) {
        let url: Vec<String> = url.split("/").map(|s| s.to_owned()).collect();
        let route = Arc::new(Route::new(url, controller));

        match method {
            HttpMethod::Get => self.get.push(route),
            HttpMethod::Post => self.post.push(route),
        };
    }
}

impl<T> std::fmt::Debug for Routes<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get.iter().for_each(|route| {
            writeln!(f, "GET  : {:#?}", route.url).unwrap();
        });

        write!(f, "\n").unwrap();

        self.post.iter().for_each(|route| {
            writeln!(f, "POST : {:#?}", route.url).unwrap();
        });

        Ok(())
    }
}
