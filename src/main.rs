use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;


#[derive(Serialize,Deserialize,Debug,Clone)]
struct Task {
    id:u64,
    name:
}

fn main() {
    println!("Hello, world!");
}
