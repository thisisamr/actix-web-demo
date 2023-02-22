#![allow(unused_imports)]
use actix_web::{web,get,App,HttpServer,Responder};
use actix_web::web::Path;

#[get("/home")]
async fn home()-> impl Responder{
    "welcome to actix-web-tuturials"
}

#[get("/hello/{firstname}/{lastname}")]

async fn hello_user(params:Path<(String,String)>)->impl Responder{
let res = format!("hello {} {}",params.0,params.1);
    res
}
#[actix_web::main]
async fn main()->std::io::Result<()> {
 HttpServer::new(||{
        App::new().service(home).service(hello_user)

    }).bind(("0.0.0.0",5000))?.run().await.and_then(|v|{
            println!("server running");
            Ok(v)
        })
    
    
}
