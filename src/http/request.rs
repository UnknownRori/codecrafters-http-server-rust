use super::HttpMethod;

#[derive(Debug)]
pub struct Request {
    user_agent: String,
    method: HttpMethod,
    path: String,
    body: String,
    param: Vec<String>,
}

impl Request {
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn param(&self) -> &Vec<String> {
        self.param.as_ref()
    }

    pub fn set_param(&mut self, param: Vec<String>) {
        self.param = param;
    }
}

#[derive(Debug)]
struct RequestBuilder {
    user_agent: Option<String>,
    method: Option<HttpMethod>,
    path: Option<String>,
    body: Option<String>,
}

impl RequestBuilder {
    fn new() -> RequestBuilder {
        RequestBuilder {
            user_agent: None,
            method: None,
            path: None,
            body: None,
        }
    }

    fn method(&mut self, method: HttpMethod) {
        self.method = Some(method);
    }

    fn path(&mut self, path: String) {
        self.path = Some(path);
    }

    fn body(&mut self, body: String) {
        self.body = Some(body);
    }

    fn user_agent(&mut self, user_agent: String) {
        self.user_agent = Some(user_agent);
    }

    fn build(self) -> Request {
        let user_agent = match self.user_agent {
            Some(val) => val,
            None => String::from(""),
        };

        let method = match self.method {
            Some(val) => val,
            None => HttpMethod::Get,
        };

        let path = match self.path {
            Some(val) => val,
            None => String::from("/"), // INFO : I have no idea what to do...
        };

        let body = match self.body {
            Some(val) => val,
            None => String::from(""),
        };

        Request {
            user_agent,
            method,
            path,
            body,
            param: vec![],
        }
    }
}

impl From<String> for Request {
    fn from(value: String) -> Self {
        let mut token = value.split("\r\n\r\n");
        let request_header = token.nth(0).expect("Invalid request header").split("\r\n");

        let mut request = RequestBuilder::new();
        for line in request_header {
            if line.starts_with("GET") || line.starts_with("POST") {
                let mut split = line.split(" ");
                let method = match split.nth(0).expect("Failed to get method") {
                    "GET" => HttpMethod::Get,
                    "POST" => HttpMethod::Post,
                    _ => HttpMethod::Get,
                };
                let path = split.nth(0).expect("Failed to get path").to_owned();

                request.path(path);
                request.method(method);
            } else if line.starts_with("User-Agent") {
                let user_agent = line.split(": ").nth(1).unwrap_or("").to_string();
                request.user_agent(user_agent);
            }
        }

        if let Some(request_body) = token.nth(0) {
            request.body(request_body.to_owned());
        }

        request.build()
    }
}
