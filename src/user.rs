use actix_cors::Cors;
use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

#[derive(SimpleObject)]
struct User {
    id: ID,
    username: String,
}

struct Query;

#[Object(extends)]
impl Query {
    async fn me(&self) -> User {
        User {
            id: "1234".into(),
            username: "Me".to_string(),
        }
    }

    #[graphql(entity)]
    async fn find_user_by_id(&self, id: ID) -> User {
        println!("testing");
        let username = if id == "1234" {
            "Me".to_string()
        } else {
            format!("User {:?}", id)
        };
        User { id, username }
    }

    #[graphql(entity)]
    async fn find_user_by_username(&self, uname: String) -> User {
        println!("testing by username");
        let username = if uname == "me" {
            "Me".to_string()
        } else {
            format!("User {:?}", uname)
        };
        User {
            id: "1234".into(),
            username,
        }
    }
}

type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn index(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:4001");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .max_age(3600),
            )
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("0.0.0.0:4001")?
    .run()
    .await
}
