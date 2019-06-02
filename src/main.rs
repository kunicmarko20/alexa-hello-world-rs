extern crate lambda_runtime as lambda;
extern crate alexa_sdk;

use lambda::{lambda, Context, error::HandlerError};
use alexa_sdk::{Request,Response};
use std::error::Error;


fn my_handler(_req: Request, _ctx: Context) -> Result<Response,HandlerError> {
    Ok(Response::simple("hello", "hello world"))
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(my_handler);

    Ok(())
}