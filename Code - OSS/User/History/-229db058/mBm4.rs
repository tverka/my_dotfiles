// extern crate iron;
#[macro_use] extern crate mime;

// extern crate Router;
use router::Router;


use iron::prelude::*;
use iron::status;



fn main() {
    let mut router = Router::new();

    router.get("/", get_form, root);
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving on http://localhost:3000...");
    Iron::new(get_from).http("localhost:3000").unwrap();
}

fn get_fm(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
    <title>GCD Calculator</title>
    <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="n"/>
        <button type="submit">Compute GCD</button>
    </form>
    "#);
    Ok(response)
}
