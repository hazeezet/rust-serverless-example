use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use rust_serverless_api::util::*;
use rust_serverless_api::{ErrorCode, ResponseError};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct GreetingInput {
    name: String,
    #[serde(default = "default_greeting")]
    greeting: String,
}

fn default_greeting() -> String {
    "Hello".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(handler)).await
}

async fn handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let (method, resource) = get_route(&event.payload);
    let stage = env::var("STAGE").unwrap_or_else(|_| "dev".to_string());

    match (method.as_str(), resource.as_str()) {
        ("GET", "/hello") => {
            let message = format!("Hello from the {} environment!", stage);
            create_api_success_response(message, Some("Greeting retrieved"))
        }
        ("GET", "/hello/{name}") => {
            let name = match get_path_param(&event.payload, "name") {
                Some(name) => name,
                None => {
                    return create_api_error_response(ResponseError::new(
                        ErrorCode::BadRequest,
                        Some("Missing name parameter"),
                    ));
                }
            };

            let message = format!("Hello, {}! Welcome to the {} environment.", name, stage);
            create_api_success_response(message, Some("Greeting retrieved"))
        }
        ("POST", "/hello") => {
            let input: GreetingInput = match parse_body(&event.payload) {
                Ok(b) => b,
                Err(e) => return create_api_error_response(e),
            };

            let message = format!("{}, {}!", input.greeting, input.name);
            create_api_success_response(message, Some("Custom greeting created"))
        }
        _ => create_api_error_response(ResponseError::new(
            ErrorCode::BadRequest,
            Some("Unknown route"),
        )),
    }
}
