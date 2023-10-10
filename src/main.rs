use std::{
    env,
    fs::File,
    io::{Read, Write},
    sync::Arc,
};

use http_server_starter_rust::http::{App, ContentType, HttpCode, Response, ResponseBuilder};

struct Data {
    directory: String,
}

fn main() {
    let conn = "127.0.0.1:4221";
    let args = env::args().collect::<Vec<String>>();

    let data = if args.len() > 2 && args[1] == "--directory" {
        Data {
            directory: args[2].to_owned(),
        }
    } else {
        Data {
            directory: String::new(),
        }
    };

    println!("Starting the server : http://{}", conn);
    let mut app = App::new(conn, data);

    app.get(
        "/",
        Arc::new(|_, _| {
            let response = ResponseBuilder::new().code(HttpCode::Ok200);
            let response: Response = response.into();
            Ok(response)
        }),
    );

    app.get(
        "/echo/{echo}/{echo}",
        Arc::new(|request, _| {
            let response = ResponseBuilder::new().code(HttpCode::Ok200).content(
                format!(
                    "{}/{}",
                    request.param().get(0).unwrap().to_owned(),
                    request.param().get(1).unwrap().to_owned(),
                ),
                http_server_starter_rust::http::ContentType::TextPlain,
            );
            Ok(response.into())
        }),
    );

    app.get(
        "/echo/{echo}",
        Arc::new(|request, _| {
            let response = ResponseBuilder::new().code(HttpCode::Ok200).content(
                request.param().get(0).unwrap().to_owned(),
                http_server_starter_rust::http::ContentType::TextPlain,
            );
            Ok(response.into())
        }),
    );

    app.get(
        "/user-agent",
        Arc::new(|request, _| {
            let response = ResponseBuilder::new().code(HttpCode::Ok200).content(
                request.user_agent().to_owned(),
                http_server_starter_rust::http::ContentType::TextPlain,
            );
            Ok(response.into())
        }),
    );

    app.get(
        "/files/{files}",
        Arc::new(|request, data| {
            let path = format!("{}/{}", data.directory, request.param().get(0).unwrap());
            let file = File::open(path);
            match file {
                Ok(mut file) => {
                    let mut buffer = String::new();
                    file.read_to_string(&mut buffer).unwrap();

                    Ok(ResponseBuilder::new()
                        .code(HttpCode::Ok200)
                        .content(buffer, ContentType::OctetStream)
                        .into())
                }
                Err(_) => Ok(ResponseBuilder::new().code(HttpCode::Err404).into()),
            }
        }),
    );

    app.post(
        "/files/{files}",
        Arc::new(|request, data| {
            let path = format!("{}/{}", data.directory, request.param().get(0).unwrap());
            let file = File::create(path);
            let body = request.body();

            let response = match file {
                Ok(mut file) => {
                    file.write_all(&body.as_bytes()).unwrap();
                    ResponseBuilder::new().code(HttpCode::Ok201)
                }
                Err(_) => ResponseBuilder::new().code(HttpCode::Err500),
            };

            Ok(response.into())
        }),
    );

    app.serve().unwrap();
}
