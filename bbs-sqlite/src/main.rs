use actix_web::{get,post,web,App,HttpServer,HttpResponse};
use rusqlite::{params, Connection, Result};
use serde_derive::Deserialize;
use askama::Template;


fn create_db()->Result<()>{
    let conn = Connection::open("bbs.db")?;

    conn.execute(
        "CREATE TABLE bbs (
            id    INTEGER PRIMARY KEY,
            data  TEXT
        )",
        (), // empty list of parameters.
    )?;

    Ok(())
}

fn get_db()->Result<Vec<String>>{
    let mut post_data:Vec<String>= Vec::new();

    let conn = Connection::open("bbs.db").ok();
    if let Some(db) = conn{
        let mut stmt = db.prepare("SELECT id, data FROM bbs").unwrap();
        let post_iter = stmt.query_map([], |row| {
            Ok(FormData {
                
                report: row.get("data")?,
            })
        }).unwrap();
        for i in post_iter {
            let post = i.unwrap();

            let result_post = post.report.unwrap_or_default();

            post_data.push(result_post);

        }
    }

    return Ok(post_data)
}

fn post_db(data: FormData)->Result<()>{
    let conn = Connection::open("bbs.db").ok();
        if let Some(db) = conn{
            db.execute(
                "INSERT INTO bbs (data) VALUES (?1)",
                params![data.report],
            ).unwrap_or_default();
        }

    return Ok(())
}


#[derive(Template)]
#[template(path="index.html")]
struct IndexTemplate{
    //Vec<String>自体がデータが空でもエラーを吐かないので、Optionにする必要がないかも。
    data:Vec<String>
}

#[derive(Deserialize)]
struct FormData {
    report: Option<String>,
}

#[get("/")]
async fn index()->HttpResponse{

    let mut result_data:Vec<String>= Vec::new();

    if let Ok(res) = get_db(){
        result_data = res;
    };
    
    let temp = IndexTemplate{
        data : result_data,
    };

    let body=temp.render().unwrap_or_default();

    HttpResponse::Ok().body(body)
}

#[post("/")]
async fn index_post(form: web::Form<FormData>)-> HttpResponse{

    if let Err(_) = post_db(form.into_inner()) {
        // エラー処理を行う
        // post_data関数がエラーだった場合エラーページに遷移する
        return HttpResponse::Ok()
                .content_type("text/html")
                .body(format!("Databese Error!"))
    }

    let mut result_data:Vec<String>= Vec::new();

    if let Ok(res) = get_db(){
        result_data = res;
    };
    
    let temp = IndexTemplate{
        data : result_data,
    };

    let body=temp.render().unwrap();

    HttpResponse::Ok().body(body)
}


//サーバーの名前
const SERVER:&str = "127.0.0.1:8080";

#[actix_web::main]
async fn main()->std::io::Result<()>{
    println!("http://{SERVER}");
    
    let _ = create_db();

    //サーバー設定
    HttpServer::new(||{
        App::new()
        .service(index)
        .service(index_post)
    })
    .bind(SERVER)?
    .run()
    .await
}
