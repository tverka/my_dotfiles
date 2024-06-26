// extern crate iron;
#[macro_use] extern crate mime;
extern crate urlencoded;

use core::num;

use iron::request;
use iron::response;
// extern crate Router;
use router::Router;


use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedBody;

use std::str::FromStr;


fn main() {
    let mut router = Router::new();

    router.get("/", get_form, root);
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving on http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
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

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e))
        }
        Ok(map) => map
    };
    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };
    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!("Value for 'n' parameter not a number: {:?}\n", unparsed));
            }
            Return Ok(response);
        }
        Ok(n) => { numbers.push(n); }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    response.set_mut(status::Ok);
response.set_mut(mime!(Text/Html; Charset=Utf8));
response.set_mut(
format!("The greatest common divisor of the numbers {:?} is <b>{}</b>\n",
numbers, d));

}

fn gcd() -> u64 {

}