// std
use std::sync::{Arc, Mutex};

// actix-web
extern crate actix_web;
use actix_web::{http, server, App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse};

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

/////////////////////////
// Get common examples //
/////////////////////////
fn get_common_examples(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let data_1 = req.state().rasa_nlu_data.clone();
    let data = data_1.lock().unwrap();
    let json = json!((*data).common_examples).to_string();

    result(Ok(HttpResponse::Ok()
              .content_type("application/json")
              .body(json)))
           .responder()
}

///////////////////////////
// Post a common example //
///////////////////////////

type PostCommonExampleJSON = rasa::CommonExample;

fn post_common_example(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let data_1 = req.state().rasa_nlu_data.clone();

    req.json()
        .from_err()
        .and_then(move |val: PostCommonExampleJSON| {
            let mut data = data_1.lock().unwrap();

            (*data).common_examples.push(val);

            Ok(HttpResponse::Ok().into())
        }).responder()
}

///////////////////////////
// Delete common example //
///////////////////////////

#[derive(Deserialize)]
struct DeleteCommonExample {
    id: usize,
}

fn delete_common_example(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let data_1 = req.state().rasa_nlu_data.clone();

    req.urlencoded::<DeleteCommonExample>()
        .from_err()
        .and_then(move |form: DeleteCommonExample| {
            let mut data = data_1.lock().unwrap();

            (*data).common_examples.remove(form.id);

            Ok(HttpResponse::Ok().into())
        }).responder()
}

fn main() {
    server::new(|| App::with_state(AppState { rasa_nlu_data: Arc::new(Mutex::new(RasaNLUData::new())) })
                    .resource("/", |r| r.f(index))
                    .resource("/common-example", |r| r.method(http::Method::GET).f(get_common_examples))
                    .resource("/common-example//", |r| r.method(http::Method::DELETE).f(delete_common_example))
                    .resource("/common-example/", |r| r.method(http::Method::POST).f(post_common_example)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}