use std::{env, sync::Arc};

use dotenv::dotenv;
use mongodb::{bson::doc, Client, Collection};
use rand::Rng;
use rocket::{
    http::Status,
    response::{status, Redirect},
    serde::{json::Json, Deserialize},
    State,
};
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UrlMapping {
    short_code: String,
    original_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserMapping {}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Shortdata<'r> {
    url: &'r str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Credentials<'r> {
    username: &'r str,
    password: &'r str,
}

struct Data {
    url_collection: Collection<UrlMapping>,
    user_collection: Collection<UserMapping>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world! :D"
}

// 62⁶ = 56,800,235,584 possible codes
async fn generate_short_code(data: &Arc<Data>) -> Result<String, status::Custom<&'static str>> {
    let code: String = (0..6)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..62); // 26 letters + 26 capitals + 10 numbers = 62
            match idx {
                0..=25 => (b'a' + idx as u8) as char, // letters 'a' to 'z'
                26..=51 => (b'A' + (idx - 26) as u8) as char, // capitals 'A' to 'Z'
                52..=61 => (b'0' + (idx - 52) as u8) as char, // numbers '0' to '9'
                _ => unreachable!(),
            }
        })
        .collect();

    let exists_filter = doc! { "short_code": &code };
    let exists_result = data.url_collection.find_one(exists_filter).await;

    match exists_result {
        Ok(result) => match result {
            Some(_) => {
                // Code already exists, generate a new one
                Box::pin(generate_short_code(data)).await
            }
            None => Ok(code), // Code is unique, return it
        },
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Database error.",
        )),
    }
}

#[post("/shorten", data = "<url>")]
async fn shorten_link(
    data: &State<Arc<Data>>,
    url: Json<Shortdata<'_>>,
) -> Result<String, status::Custom<&'static str>> {
    let mut link = url.0.url;

    link = match link.strip_prefix("http://") {
        Some(link) => link,
        None => link,
    };
    link = match link.strip_prefix("https://") {
        Some(link) => link,
        None => link,
    };

    if link == "" {
        return Err(status::Custom(Status::BadRequest, "Please provide a URL."));
    }

    let short_code = Box::pin(generate_short_code(&data)).await;

    let new_mapping = UrlMapping {
        short_code: short_code.clone()?,
        original_url: link.to_string(),
    };

    let filter = doc! { "original_url": link };
    let exists_result = data.url_collection.find_one(filter).await;

    match exists_result {
        Ok(result) => match result {
            Some(result) => return Ok(result.short_code),
            None => {}
        },
        Err(e) => {
            println!("{}", e);
            return Err(status::Custom(
                Status::InternalServerError,
                "Database error.",
            ));
        }
    };

    let result = data.url_collection.insert_one(new_mapping.clone()).await;

    match result {
        Ok(_) => Ok(new_mapping.short_code),
        Err(e) => {
            println!("{}", e);
            Err(status::Custom(
                Status::InternalServerError,
                "Database error.",
            ))
        }
    }
}

#[delete("/<code>", data = "<credentials>")]
async fn delete_link(
    code: &str,
    data: &State<Arc<Data>>,
    credentials: Json<Credentials<'_>>,
) -> status::Custom<&'static str> {
    let filter = doc! { "username": credentials.0.username, "password": credentials.0.password };
    let result = data.user_collection.find_one(filter).await;

    match result {
        Ok(user) => match user {
            Some(_) => {
                let filter = doc! { "short_code": code };
                let result = data.url_collection.delete_one(filter).await;

                match result {
                    Ok(result) => {
                        if result.deleted_count == 0 {
                            status::Custom(
                                Status::InternalServerError,
                                "Unable to find or delete the shortcode.",
                            )
                        } else {
                            status::Custom(Status::Ok, "Short code deleted.")
                        }
                    }
                    Err(_) => status::Custom(
                        Status::InternalServerError,
                        "Unable to find or delete the shortcode.",
                    ),
                }
            }
            None => status::Custom(Status::BadRequest, "Wrong username and, or password."),
        },
        Err(_) => status::Custom(
            Status::InternalServerError,
            "Error while checking credentials.",
        ),
    }
}

#[get("/<code>")]
async fn get_link(
    code: &str,
    data: &State<Arc<Data>>,
) -> Result<Redirect, status::Custom<&'static str>> {
    let filter = doc! { "short_code": code };
    let result = data.url_collection.find_one(filter).await;

    match result {
        Ok(link) => match link {
            Some(obj) => {
                let url: &str = &obj.original_url;
                Ok(Redirect::to(format!("https://{}", url)))
            }
            None => Err(status::Custom(
                Status::NotFound,
                "No URL assigned to that shortcode.",
            )),
        },
        Err(_) => Err(status::Custom(
            Status::NotFound,
            "No URL assigned to that shortcode.",
        )),
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let client_uri = env::var("MONGODB_URI").expect("MONGODB_URI not found in .env");
    let client = Client::with_uri_str(client_uri)
        .await
        .expect("Failed to initialize mongodb client.");
    let database = client.database(&env::var("MONGODB_DB").expect("MONGODB_DB not found in .env"));
    let url_collection: Collection<UrlMapping> = database.collection(
        &env::var("MONGODB_URL_COLLECTION").expect("MONGODB_URL_COLLECTION not found in .env"),
    );
    let user_collection: Collection<UserMapping> = database.collection(
        &env::var("MONGODB_USER_COLLECTION").expect("MONGODB_USER_COLLECTION not found in .env"),
    );

    rocket::build()
        .manage(Arc::new(Data {
            url_collection,
            user_collection,
        }))
        .mount("/", routes![index, shorten_link, get_link, delete_link])
}
