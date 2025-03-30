use std::collections::HashMap;

use actix_web::{
    middleware,
    web::{self, Data},
    App, Error, HttpResponse, HttpServer,
};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, GraphQLObject, RootNode};
use juniper_actix::{graphql_handler, playground_handler};


#[derive(Clone, GraphQLObject)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Default)]
pub struct Database {
    pub users: HashMap<i32, User>,
}
impl Database {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert(1, User { id: 1, name: "Alice".to_string() });
        users.insert(2, User { id: 2, name: "Bob".to_string() });
        Database { users }
    }

    pub fn get_user(&self, id: &i32) -> Option<&User> {
        self.users.get(id)
    }
}

impl juniper::Context for Database {}


pub struct Query;
#[graphql_object(context = Database)]
impl Query {
    fn api_version() -> String {
        "1.0".to_string()
    }

    fn user(
        context: &Database,
        #[graphql(description = "id of the user")] id: i32
    ) -> Option<&User> {
        context.get_user(&id)
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
}

async fn  playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}

async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = Database::new();
    graphql_handler(&schema, &context, req, payload).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema()))
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(playground_route)),
            )
    });

    let url = "127.0.0.1:4000";
    println!("Starting server at http://{}", url);
    server.bind(url).unwrap().run().await
    // RUST_LOG=debug cargo run
}
