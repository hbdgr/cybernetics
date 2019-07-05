use server::handler;
use database::connection_pool;

use rocket::fairing::AdHoc;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};


// #[options("/")]
// fn cors<'a>() -> &'a str {
//     "Hello CORS"
// }

fn cors_options() -> rocket_cors::Cors {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:8080"]);
    assert!(failed_origins.is_empty());

    // You can also deserialize this
    rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete, Method::Options]
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

pub fn create_routes() {
    rocket::ignite()
        .manage(connection_pool::init_pool())
        .manage(cors_options())
        .mount(
            "/",
            rocket_cors::catch_all_options_routes()
        )
        .mount(
            "/",
            routes![
                handler::auth_post
            ],
        )
        .mount(
            "/objects",
            routes![
                handler::all,
                handler::get,
                handler::post,
                handler::put,
                handler::delete,
            ],
        )
        .attach(cors_options())
        .attach(AdHoc::on_request("Post Request", |req, data| {
            if req.method() == Method::Post {
                if data.peek_complete() {
                    let s = String::from_utf8(data.peek().to_vec())
                        .expect("Found invalid UTF-8");
                    println!("    => Request: DATA {}", s);
                }
            }
        }))
        .launch();
}
