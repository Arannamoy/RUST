use actix_web::{HttpServer, App, web::{self, Path, Json}};
use serde::{Deserialize};


#[derive(Deserialize)]
struct EntityId{
    id:i64,
}


#[derive(Clone)]
struct  FinalUSer{
    id:i64,
    user_name:String,
    full_name:String
}
struct AppState{
    users:std::sync::RwLock<Vec<FinalUSer>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let app_data = web::Data::new(AppState {
        users:std::sync::RwLock::new(vec![
            FinalUSer{id:1,user_name:"user1".to_string(),full_name:"user1".to_string()},
            FinalUSer{id:2,user_name:"user2".to_string(),full_name:"user2".to_string()},
            FinalUSer{id:3,user_name:"user3".to_string(),full_name:"user3".to_string()}
        ]),
    } );
    HttpServer::new(move||{
        App::new()
            .app_data(app_data.clone())
            .service(web::scope("/v1")
                .service(web::resource("/user/{id}").route(web::get().to(get_user_name)))
            )
    })   
    .bind(("127.0.0.1",8001))?
    .run()
    .await
}


async fn get_user_name(app_data: web::Data<AppState>,params:Path<EntityId>) -> String {
    let users = app_data.get_ref().users.read().unwrap();
    let user=users.iter().find(|x| x.id==params.id).unwrap().clone().user_name;
    user
}