use database::connection_pool;
use diesel::result::Error;
use server::handle_objects;
use server::handle_relations;

use rocket::fairing::AdHoc;
use rocket::http::{Method, Status};
use rocket::Rocket;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

// #[options("/")]
// fn cors<'a>() -> &'a str {
//     "Hello CORS"
// }

fn cors_options() -> rocket_cors::Cors {
    let port_allowed = 65320;
    let host_allowed = format!("http://localhost:{}", port_allowed.to_string());
    println!("CORS: Host allowed to connect: {}", host_allowed);

    let (allowed_origins, failed_origins) = AllowedOrigins::some(&[&host_allowed]);
    assert!(failed_origins.is_empty());

    // You can also deserialize this
    rocket_cors::Cors {
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
        allowed_headers: AllowedHeaders::all(),
        // allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
}

pub fn create_routes() -> Rocket {
    rocket::ignite()
        .manage(connection_pool::init_pool())
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

pub fn launch_routes() {
    create_routes().launch();
}

pub fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}
