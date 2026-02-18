use actix_web::{web,App,HttpServer,Responder};
use std::sync::Mutex;

#[derive(Clone)]
struct Messenger{
    message:String
}
// #[derive(Clone)]
struct MutableState{
    messenger:Mutex<Messenger>
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let app_data=web::Data::new(MutableState{
        messenger:Mutex::new(Messenger{message:"Hello world".to_string()})
    });
    HttpServer::new(move||{
        App::new()
        .app_data(app_data.clone())
        .route("/",web::get().to(index))
        .service(
            web::scope("/v1")
            .route("/",web::get().to(index))
            .route("/post",web::post().to(index))
        )
        
        .route("/appState",web::post().to(update))
        .route("/appState",web::get().to(get))
    })
    .bind(("127.0.0.1",8001))?
    .run()
    .await
}



async fn index()->&'static str{
    "hello world"
}

async fn update(app_data:web::Data<MutableState>)->impl Responder{
    let mut messenger=app_data.messenger.lock().unwrap();
    messenger.message=format!("{} world",messenger.message);
    "Updated successfully".to_string()
}


async fn get(data: web::Data<MutableState>) -> impl Responder {
    let messenger = data.messenger.lock().unwrap();
    format!("Current Message: {}", messenger.message)
}