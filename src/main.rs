// std
use std::sync::{Arc, Mutex};

// actix-web
extern crate actix_web;
use actix_web::*;
use actix_web::http::Method;

// futures
extern crate futures;
use futures::future::{Future, result};

// serde
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

// internal
mod rasa;
use rasa::RasaNLUData;

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
        },

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
                }).responder()
        },

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

                    // NOTE: Not checking array bounds
                    (*data).common_examples.remove(form.id);

                    Ok(HttpResponse::Ok().into())
                }).responder()
        },
        _ => {
            result(Ok(HttpResponse::MethodNotAllowed().into())).responder()
        }
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
        },

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
                }).responder()
        },

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

                    // NOTE: Not checking array bounds
                    (*data).regex_features.remove(form.id);

                    Ok(HttpResponse::Ok().into())
                }).responder()
        },
        _ => {
            result(Ok(HttpResponse::MethodNotAllowed().into())).responder()
        }
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
        },

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
                }).responder()
        },

        // Delete a common example
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

                    // NOTE: Not checking array bounds
                    (*data).entity_synonyms.remove(form.id);

                    Ok(HttpResponse::Ok().into())
                }).responder()
        },
        _ => {
            result(Ok(HttpResponse::MethodNotAllowed().into())).responder()
        }
    }
}

fn main() {
    server::new(|| App::with_state(AppState { rasa_nlu_data: Arc::new(Mutex::new(RasaNLUData::new())) })
                    .resource("/", |r| r.f(index))
                    .resource("/common-example", |r| r.f(common_example))
                    .resource("/regex-feature", |r| r.f(regex_feature))
                    .resource("/entity-synonym", |r| r.f(entity_synonym)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}