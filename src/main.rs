// std
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

// actix-web
extern crate actix_web;
use actix_web::http::Method;
use actix_web::middleware::cors::Cors;
use actix_web::*;

// clap
extern crate clap;
use clap::Arg;

// env_logger
extern crate env_logger;

// futures
extern crate futures;
use futures::future::{result, Future};

// serde
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

// internal
mod rasa;
use rasa::*;

const SAVE_FILE_PATH: &str = "rasa_nlu_data.json";

// This is where's we are going to manipulate Rasa NLU data
struct AppState {
    rasa_nlu_data: Arc<Mutex<RasaNLUData>>,
}

// Index
fn index(_req: &HttpRequest<AppState>) -> &'static str {
    "Hello world!\n"
}

////////////////////
// Common Example //
////////////////////

fn common_example(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    match req.method() {
        // Get common examples
        &Method::GET => {
            let data_1 = req.state().rasa_nlu_data.clone();
            let data = data_1.lock().unwrap();
            let json = json!((*data).common_examples).to_string();

            result(Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(json)))
                .responder()
        }

        // Post a common example
        &Method::POST => {
            type PostCommonExample = rasa::CommonExample;

            let data_1 = req.state().rasa_nlu_data.clone();

            req.json()
                .from_err()
                .and_then(move |val: PostCommonExample| {
                    let mut data = data_1.lock().unwrap();

                    (*data).common_examples.push(val);

                    Ok(HttpResponse::Ok().into())
                })
                .responder()
        }

        // Update a common example
        &Method::PUT => {
            #[derive(Deserialize)]
            struct PutCommonExample {
                id: usize,
                text: String,
                intent: String,
                entities: Vec<Entity>,
            }

            let data_1 = req.state().rasa_nlu_data.clone();

            req.json()
                .from_err()
                .and_then(move |val: PutCommonExample| {
                    let mut data = data_1.lock().unwrap();

                    // Check array bound
                    if val.id >= (*data).common_examples.len() {
                        return Ok(HttpResponse::BadRequest().into());
                    }

                    (*data).common_examples.remove(val.id);
                    (*data).common_examples.insert(
                        val.id,
                        CommonExample {
                            text: val.text,
                            intent: val.intent,
                            entities: val.entities,
                        },
                    );

                    Ok(HttpResponse::Ok().into())
                })
                .responder()
        }

        // Delete a common example
        &Method::DELETE => {
            #[derive(Deserialize)]
            struct DeleteCommonExample {
                id: usize,
            }

            let data_1 = req.state().rasa_nlu_data.clone();

            req.urlencoded::<DeleteCommonExample>()
                .from_err()
                .and_then(move |form: DeleteCommonExample| {
                    let mut data = data_1.lock().unwrap();

                    // Check array bound
                    if form.id >= (*data).common_examples.len() {
                        return Ok(HttpResponse::BadRequest().into());
                    }

                    (*data).common_examples.remove(form.id);

                    Ok(HttpResponse::Ok().into())
                })
                .responder()
        }
        _ => result(Ok(HttpResponse::MethodNotAllowed().into())).responder(),
    }
}

///////////////////
// Regex Feature //
///////////////////

fn regex_feature(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    match req.method() {
        // Get regex features
        &Method::GET => {
            let data_1 = req.state().rasa_nlu_data.clone();
            let data = data_1.lock().unwrap();
            let json = json!((*data).regex_features).to_string();

            result(Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(json)))
                .responder()
        }

        // Post a regex feature
        &Method::POST => {
            type PostRegexFeature = rasa::RegexFeature;

            let data_1 = req.state().rasa_nlu_data.clone();

            req.json()
                .from_err()
                .and_then(move |val: PostRegexFeature| {
                    let mut data = data_1.lock().unwrap();

                    (*data).regex_features.push(val);

                    Ok(HttpResponse::Ok().into())
                })
                .responder()
        }

        // Update a regex feature
        &Method::PUT => {
            #[derive(Deserialize)]
            struct PutRegexFeature {
                id: usize,
                name: String,
                pattern: String,
            }

            let data_1 = req.state().rasa_nlu_data.clone();

            req.json()
                .from_err()
                .and_then(move |val: PutRegexFeature| {
                    let mut data = data_1.lock().unwrap();

                    // Check array bound
                    if val.id >= (*data).regex_features.len() {
                        return Ok(HttpResponse::BadRequest().into());
                    }

                    (*data).regex_features.remove(val.id);
                    (*data).regex_features.insert(
                        val.id,
                        RegexFeature {
                            name: val.name,
                            pattern: val.pattern,
                        },
                    );

                    Ok(HttpResponse::Ok().into())
                })
                .responder()
        }

        // Delete a regex feature
        &Method::DELETE => {
            #[derive(Deserialize)]
            struct DeleteRegexFeature {
                id: usize,
            }

            let data_1 = req.state().rasa_nlu_data.clone();

            req.urlencoded::<DeleteRegexFeature>()
                .from_err()
                .and_then(move |form: DeleteRegexFeature| {
                    let mut data = data_1.lock().unwrap();

                    // Check array bound
                    if form.id >= (*data).regex_features.len() {
                        return Ok(HttpResponse::BadRequest().into());
                    }

                    (*data).regex_features.remove(form.id);

                    Ok(HttpResponse::Ok().into())
                })
                .responder()
        }
        _ => result(Ok(HttpResponse::MethodNotAllowed().into())).responder(),
    }
}

////////////////////
// Entity Synonym //
////////////////////

fn entity_synonym(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    match req.method() {
        // Get entity synonyms
        &Method::GET => {
            let data_1 = req.state().rasa_nlu_data.clone();
            let data = data_1.lock().unwrap();
            let json = json!((*data).entity_synonyms).to_string();

            result(Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(json)))
                .responder()
        }

        // Post a entity synonym
        &Method::POST => {
            type PostEntitySynonym = rasa::EntitySynonym;

            let data_1 = req.state().rasa_nlu_data.clone();

            req.json()
                .from_err()
                .and_then(move |val: PostEntitySynonym| {
                    let mut data = data_1.lock().unwrap();

                    (*data).entity_synonyms.push(val);

                    Ok(HttpResponse::Ok().into())
                })
                .responder()
        }

        // Update a entity synonym
        &Method::PUT => {
            #[derive(Deserialize)]
            struct PutEntitySynonym {
                id: usize,
                value: String,
                synonyms: Vec<String>,
            }

            let data_1 = req.state().rasa_nlu_data.clone();

            req.json()
                .from_err()
                .and_then(move |val: PutEntitySynonym| {
                    let mut data = data_1.lock().unwrap();

                    // Check array bound
                    if val.id >= (*data).entity_synonyms.len() {
                        return Ok(HttpResponse::BadRequest().into());
                    }

                    (*data).entity_synonyms.remove(val.id);
                    (*data).entity_synonyms.insert(
                        val.id,
                        EntitySynonym {
                            value: val.value,
                            synonyms: val.synonyms,
                        },
                    );

                    Ok(HttpResponse::Ok().into())
                })
                .responder()
        }

        // Delete a entity synonym
        &Method::DELETE => {
            #[derive(Deserialize)]
            struct DeleteEntitySynonym {
                id: usize,
            }

            let data_1 = req.state().rasa_nlu_data.clone();

            req.urlencoded::<DeleteEntitySynonym>()
                .from_err()
                .and_then(move |form: DeleteEntitySynonym| {
                    let mut data = data_1.lock().unwrap();

                    // Check array bound
                    if form.id >= (*data).entity_synonyms.len() {
                        return Ok(HttpResponse::BadRequest().into());
                    }

                    (*data).entity_synonyms.remove(form.id);

                    Ok(HttpResponse::Ok().into())
                })
                .responder()
        }
        _ => result(Ok(HttpResponse::MethodNotAllowed().into())).responder(),
    }
}

fn save(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    match req.method() {
        &Method::POST => {
            let data_1 = req.state().rasa_nlu_data.clone();
            let data = data_1.lock().unwrap();
            let mut saved_file = match File::create(SAVE_FILE_PATH) {
                Ok(f) => f,
                Err(_) => return result(Ok(HttpResponse::InternalServerError().into())).responder(),
            };

            let rasa_nlu = RasaNLU {
                rasa_nlu_data: (*data).clone(),
            };
            let json = serde_json::to_vec(&rasa_nlu).unwrap();
            let res = match saved_file.write_all(&json) {
                Ok(_) => Ok(HttpResponse::Ok().into()),
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            };

            result(res).responder()
        }
        _ => result(Ok(HttpResponse::MethodNotAllowed().into())).responder(),
    }
}

fn data(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    match req.method() {
        &Method::GET => {
            let data_1 = req.state().rasa_nlu_data.clone();
            let data = data_1.lock().unwrap();
            let json = serde_json::to_string(&*data).unwrap();
            let res = HttpResponse::Ok()
                .content_type("application/json")
                .body(json);
            result(Ok(res)).responder()
        }
        _ => result(Ok(HttpResponse::MethodNotAllowed().into())).responder(),
    }
}

fn main() {
    env_logger::init();

    // Parse command-line options
    let matches = clap::App::new("Rasa NLU API")
        .version("0.0.1")
        .author("Jacky Boen <jacky@veand.co>")
        .about("Rasa NLU API for editing training dataset")
        .arg(
            Arg::with_name("data")
                .short("d")
                .long("data")
                .value_name("FILE")
                .help("Sets data file to load from")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("cors")
                .long("cors")
                .help("Allow CORS")
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    // Check data file path if it's set in the program arguments
    let save_file_path = matches.value_of("data").unwrap_or(SAVE_FILE_PATH);

    // Open saved file
    let rasa_nlu = match File::open(save_file_path) {
        Ok(f) => {
            // Parse JSON from data
            match serde_json::from_reader(f) {
                Ok(data) => {
                    println!("Using data from {}", save_file_path);
                    data
                }
                Err(_) => {
                    println!("Using empty data");
                    RasaNLU::new()
                }
            }
        }
        Err(_) => {
            println!("Using empty data");
            RasaNLU::new()
        }
    };

    // Run server
    if matches.is_present("cors") {
        server::new(move || {
            App::with_state(AppState {
                rasa_nlu_data: Arc::new(Mutex::new(rasa_nlu.rasa_nlu_data.clone())),
            }).configure(|app| {
                Cors::for_app(app)
                    .resource("/", |r| r.f(index))
                    .resource("/common-example", |r| r.f(common_example))
                    .resource("/regex-feature", |r| r.f(regex_feature))
                    .resource("/entity-synonym", |r| r.f(entity_synonym))
                    .resource("/save", |r| r.f(save))
                    .resource("/data", |r| r.f(data))
                    .register()
            })
        }).bind("127.0.0.1:8088")
            .unwrap()
            .run();
    } else {
        server::new(move || {
            App::with_state(AppState {
                rasa_nlu_data: Arc::new(Mutex::new(rasa_nlu.rasa_nlu_data.clone())),
            }).resource("/", |r| r.f(index))
                .resource("/common-example", |r| r.f(common_example))
                .resource("/regex-feature", |r| r.f(regex_feature))
                .resource("/entity-synonym", |r| r.f(entity_synonym))
                .resource("/save", |r| r.f(save))
                .resource("/data", |r| r.f(data))
        }).bind("127.0.0.1:8088")
            .unwrap()
            .run();
    }
}