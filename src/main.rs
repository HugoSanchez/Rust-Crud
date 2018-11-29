#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

// Rocket
use rocket_contrib::{ Json, Value };

//MongoDB
use mongodb::{ Client, ThreadedClient };
use mongodb::db::ThreadedDatabase;

// Bson
use bson::oid::ObjectId;

// Hero mod & struct
mod hero;
use hero::{Hero};

// db mod
mod db;

#[get("/")]
fn read() -> Json<Value> {
    // Connect to remote MongoDB database
    let client = Client::with_uri(db::DATABASE_URL)
        .ok().expect("Failed to initialize client");

    // Database Authentication
    let db = client.db("rustcrud");
    db.auth(db::DATABASE_USER, db::DATABASE_PASSWORD)
        .ok().expect("AUTH Failed");

    // Connecting to a specific collection
    let coll = db.collection("test");

    // Retrieving from DB
    let cursor = coll.find(None, None).ok().expect("Failed to execute find.");

    // Create a Vector with all items in DB
    let heroes: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();

    // Send back heroes collection
    Json(json!({"heroes": heroes}))
}

#[post("/", format = "application/json", data = "<hero>")]
fn create(hero: Json<Hero>) -> Json<Value> {
    // Parse HERO instance
    let hero_instance = Hero { ..hero.into_inner()};
    let hero_instance = Json(hero_instance);
    let hero_doc = doc!{
        "name" => hero_instance.name.to_string(),
        "identity" => hero_instance.identity.to_string(),
        "superpower" => hero_instance.superpower.to_string(),
        "hometown" => hero_instance.hometown.to_string(),
        "age" => hero_instance.age.to_string(),
    };

    // Initialize success variable to triger response
    let response_status: String;

    // Connect to remote MongoDB database
    let client = Client::with_uri(db::DATABASE_URL)
        .ok().expect("Failed to initialize client");

    // Database Authentication
    let db = client.db("rustcrud");
    db.auth(db::DATABASE_USER, db::DATABASE_PASSWORD)
        .ok().expect("AUTH Failed");

    // Connecting to a specific collection
    let coll = db.collection("test");

    // Insterting the User instance into DB
    match coll.insert_one(hero_doc, None) {
        Ok(_) => response_status = "Success".to_string(),
        Err(_) => response_status = "Unable to store item".to_string(),
    };

    // Send back status feedback as JSON
    Json(json!({"Status": response_status}))
}

#[put("/<id>", data="<hero>")]
fn update(id: String, hero: Json<Hero>) -> Json<Hero>{
    hero
}

#[delete("/<id>")]
fn delete(id: String) -> Json<Value> {

    // Connect to remote MongoDB database
    let client = Client::with_uri(db::DATABASE_URL)
        .ok().expect("Failed to initialize client");

    // Database Authentication
    let db = client.db("rustcrud");
    db.auth(db::DATABASE_USER, db::DATABASE_PASSWORD)
        .ok().expect("AUTH Failed");

    // Connecting to a specific collection
    let coll = db.collection("test");

    // Initialize success variable to triger response
    let response_status: String;

    // Match the user id to an bson ObjectId
    // let object_id: &str = &id;
    // let object_id = ObjectId::with_string(object_id);

    // Delete item from db
    match coll.delete_one(doc! {"$oid" => id}, None) {
        Ok(_) => response_status = "Success".to_string(),
        Err(_) => response_status = "Unable to delete item".to_string(),
    };

    // Send back response status
    Json(json!({"status": response_status}))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![read, create, update, delete])
        .launch();
}
