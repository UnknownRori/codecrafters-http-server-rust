use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock},
};

use super::{FnController, HttpMethod, Request, Routes};

#[derive(Debug)]
pub struct App<T>
where
    T: Sync + Send + 'static,
{
    routes: Routes<T>,
    application_data: T,
    ip: String,
}

impl<T> App<T>
where
    T: Sync + Send + 'static,
{
    /// Create TCP Listener and create HTTP/1.1 server and bind to specific ip
    pub fn new(ip: &str, application_data: T) -> Self {
        Self {
            routes: Routes::new(),
            ip: ip.to_owned(),
            application_data,
        }
    }

    pub fn get(&mut self, url: &str, controller: FnController<T>) {
        self.routes
            .register_route(HttpMethod::Get, url.to_owned(), controller);
    }

    pub fn post(&mut self, url: &str, controller: FnController<T>) {
        self.routes
            .register_route(HttpMethod::Post, url.to_owned(), controller);
    }

    pub fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let routes = Arc::new(RwLock::new(self.routes));
        let application_data = Arc::new(RwLock::new(self.application_data));
        let listener = TcpListener::bind(self.ip).unwrap();

        for stream in listener.incoming() {
            let routes = Arc::clone(&routes);
            let application_data = Arc::clone(&application_data);

            let stream = stream.unwrap();
            std::thread::spawn(move || handle(stream, routes, application_data));
        }

        Ok(())
    }
}

fn handle<T>(
    mut stream: TcpStream,
    routes: Arc<RwLock<Routes<T>>>,
    application_data: Arc<RwLock<T>>,
) {
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read the stream");

    let converted_request = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
    let mut request: Request = converted_request.into();

    {
        let routes = routes.read().unwrap();
        let route = routes.resolve(&mut request).unwrap();
        let request = Arc::new(RwLock::new(request));
        let controller = (*route).controller();
        {
            let request = Arc::clone(&request);
            let request = request.read().unwrap();

            let application_data = application_data.read().unwrap();

            resolve(&mut stream, controller, &application_data, &request).unwrap();
        }
    }
}

fn resolve<T>(
    stream: &mut TcpStream,
    controller: &FnController<T>,
    data: &T,
    request: &Request,
) -> Result<(), Box<dyn std::error::Error>> {
    let result_controller = controller(&request, &data).unwrap();
    let result = result_controller.to_string();

    println!("Sending Response : {:#?}", &result);
    stream
        .write(&result.as_bytes())
        .expect("Failed to send a response");

    stream.flush().unwrap();

    Ok(())
}
