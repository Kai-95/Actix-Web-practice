use log::info;
use actix_web::{Responder, HttpResponse, web, get, post};
mod data;

#[get("/post")]
pub async fn index() -> impl Responder{
    let post = data::get_all();
    let mut body_str: String= "".to_string();
    body_str += include_str!("../static/header.html");
    for item in &post {				
        body_str += &format!("<div><a href=\"/posts/{}\">", item.id);
        body_str += &format!("<div>{} {}</div>", item.sender, item.posted);
        body_str += &format!("<div><p>{}</p></div>", 
            item.content.replace("\n", "<br />"));
        body_str += "</a></div>";
    }
    body_str += "<div><a href=\"/posts/new\">作成</a></div>";
    body_str += include_str!("../static/footer.html");
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body_str)
    //info!("Called index");
    //HttpResponse::Ok().body("Called index")
}

pub async fn not_found() -> impl Responder{
    HttpResponse::NotFound().body("page not found!")
}

