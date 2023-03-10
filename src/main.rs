#![feature(proc_macro_hygiene, decl_macro)]  

use std::time::Instant;     // Importing the `Instant` module from the `std::time` crate.

use juniper_rocket;          
use rocket::{response::content, State};     
//Importing the necessary modules from the `rocket` and `juniper_rocket` crates.

mod db;                     // Declaring a module for database connection.
mod models;                 // Declaring a module for model definitions.
mod schema;                 // Declaring a module for GraphQL schema definition.

use db::DataContext;        // Importing DataContext struct from db module.
use schema::*;              // Importing all functions and types defined in the schema module.


#[rocket::get("/")]        //Setting up the endpoint for graphiql.
fn graphiql() -> content::Html<String> {    // Defining the function that returns html string for GraphiQL interface.
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/grapqhl?<request>")]      // Setting up the endpoint for GraphQL queries.
fn get_graphql_handler(
    context: State<DataContext>,  // Defining a `State` parameter that will inject the `DataContext` struct into the function.
    request: juniper_rocket::GraphQLRequest, // Defining a `GraphQLRequest` parameter that contains query/mutation and variables
    schema: State<Schema>,          // Defining a `State` parameter to provide access to the GraphQL schema created.
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)   // execute the GraphQL request using the schema and return the response object.
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<DataContext>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {   // Accepts POST requests for executing graphql queries and will return JSON format responses
    let start = Instant::now();             //start timer using instant function from std.
    let response = request.execute(&schema, &context);  //execute the request and assign response to variable.
    println!("Request took {:?}", start.elapsed());  // print the time elapsed during completing the request.
    response    // returning the response in JSON format.
}

fn main() {
    rocket::ignite()
        .manage(DataContext::init().unwrap())  // Initializing DataContext via `.manage()` method and binding it to the application's Rocket state
        .manage(Schema::new(Query, Mutation))  // Initializing and binding `Schema` instance
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )   // Attaching the route handlers for different endpoints for GraphQL and GraphiQL interfaces.
        .launch();  // Launches the Rocket application.
}
