use crate::{ApiResponse, ResponseError};
use aws_lambda_events::{
    apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse},
    encodings::Body as LambdaBody,
    http::HeaderMap,
};
use lambda_runtime::Error;
use serde::{de::DeserializeOwned, Serialize};

/// Build response headers with CORS
fn response_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert(
        "Access-Control-Allow-Headers",
        "Content-Type".parse().unwrap(),
    );
    headers.insert(
        "Access-Control-Allow-Methods",
        "GET,POST,PUT,DELETE,OPTIONS".parse().unwrap(),
    );
    headers
}

/// Create a standardized ApiGatewayProxyResponse
pub fn create_api_response<T: Serialize>(
    status_code: i64,
    response: &ApiResponse<T>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let mut res = ApiGatewayProxyResponse::default();
    res.status_code = status_code;
    res.headers = response_headers();
    res.multi_value_headers = HeaderMap::new();
    res.body = Some(LambdaBody::Text(serde_json::to_string(response)?));
    res.is_base64_encoded = false;
    Ok(res)
}

/// Create error ApiGatewayProxyResponse from ResponseError
pub fn create_api_error_response(error: ResponseError) -> Result<ApiGatewayProxyResponse, Error> {
    let response = ApiResponse::<()> {
        status_code: error.status_code,
        error: Some(error.code),
        message: error.message,
        data: None,
    };
    create_api_response(error.status_code as i64, &response)
}

/// Create success ApiGatewayProxyResponse
pub fn create_api_success_response<T: Serialize>(
    data: T,
    message: Option<&str>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let response = ApiResponse {
        status_code: 200,
        error: None,
        message: message.unwrap_or("successful").to_string(),
        data: Some(data),
    };
    create_api_response(200, &response)
}

/// Extract (HTTP_METHOD, resource_template) from a proxy request.
/// Returns e.g. `("GET", "/hello/{name}")`.
pub fn get_route(request: &ApiGatewayProxyRequest) -> (String, String) {
    let method = request.http_method.to_string();
    let resource = request.resource.clone().unwrap_or_default();
    (method, resource)
}

/// Get a path parameter set by API Gateway (e.g. `{name}`).
pub fn get_path_param(request: &ApiGatewayProxyRequest, key: &str) -> Option<String> {
    request.path_parameters.get(key).map(|v| v.to_string())
}

/// Get a single query-string parameter.
pub fn get_query_param(request: &ApiGatewayProxyRequest, key: &str) -> Option<String> {
    request
        .query_string_parameters
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| v.to_string())
}

/// Deserialize the JSON body from the request into `T`.
pub fn parse_body<T: DeserializeOwned>(
    request: &ApiGatewayProxyRequest,
) -> Result<T, ResponseError> {
    let body = request.body.as_deref().unwrap_or("{}");
    serde_json::from_str::<T>(body).map_err(|e| {
        ResponseError::new(
            crate::ErrorCode::BadRequest,
            Some(&format!("Invalid request body: {e}")),
        )
    })
}
