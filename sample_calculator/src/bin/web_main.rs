use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormulaParameters{
    formula: String,
}

fn main(){
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/calc", web::post().to(post_formula))
    });

    println!("Serving on http://localhost:3000...");
    server.bind("127.0.0.1:3000").expect("error binding server to adress")
          .run().expect("error running server");
}

fn get_index() -> HttpResponse{
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>Calculator</title>
                <form action="/calc" method="post">
                <input type="text" name="formula"/>
                <button type="submit"> Compute Formula</button>
                </form>
            "#,
        )
}



fn post_formula(form: web::Form<FormulaParameters>) -> HttpResponse{
    let result = sample_calculator::calculator::polish_notation::calc_from_formula(&form.formula);
    let response = match result{
        Ok(answer) => format!("The answer is {}", answer),
        Err(msg) => format!("Error: {}", msg),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}