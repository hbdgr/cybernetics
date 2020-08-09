use database::connection_pool::Pool;
use std::env;

use diesel::result::Error;
use server::handle_objects;
use server::handle_relations;

use rocket::fairing::AdHoc;
use rocket::http::{Method, Status};
use rocket::Rocket;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

// #[options("/")]
// fn cors<'a>() -> &'a str {
//     "Hello CORS"
// }

fn cors_options() -> rocket_cors::Cors {
    let allowed_port =
        env::var("ROCKET_CORS_ALLOWED_PORT").expect("ROCKET_CORS_ALLOWED_PORT must be set");
    let allowed_addr =
        env::var("ROCKET_CORS_ALLOWED_ADDR").expect("ROCKET_CORS_ALLOWED_ADDR must be set");

    let host_allowed;
    if allowed_port == "80" {
        host_allowed = [format!("{}", allowed_addr)];
    } else {
        host_allowed = [format!("{}:{}", allowed_addr, allowed_port.to_string())];
    }

    let allowed_origins = AllowedOrigins::some_regex(&host_allowed);
    println!("CORS: allowed_origins: {:?}", allowed_origins);

    // You can also deserialize this
    CorsOptions {
        allowed_origins: allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Options,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
            "Content-Type",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS ERROR")
}

pub fn create_routes(db_pool: Pool) -> Rocket {
    rocket::ignite()
        .manage(db_pool)
        .manage(cors_options())
        .mount("/", rocket_cors::catch_all_options_routes())
        .mount("/", routes![handle_objects::auth_post])
        .mount(
            "/objects",
            routes![
                handle_objects::all,
                handle_objects::get,
                handle_objects::post,
                handle_objects::put,
                handle_objects::delete,
            ],
        )
        .mount(
            "/relations",
            routes![
                handle_relations::all,
                handle_relations::post,
                handle_relations::get,
                handle_relations::put,
                handle_relations::delete
            ],
        )
        .attach(cors_options())
        .attach(AdHoc::on_request("Post Request", |req, data| {
            if req.method() == Method::Post {
                if data.peek_complete() {
                    let s = String::from_utf8(data.peek().to_vec()).expect("Found invalid UTF-8");
                    println!("    => Request: DATA {}", s);
                }
            }
        }))
}

pub fn launch_routes(db_pool: Pool) {
    create_routes(db_pool).launch();
}

pub fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}
