use crate::{config::Config, error::ApiError};
use actix_web::{http::StatusCode, web, Responder};
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use snailquote::unescape;

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    address: String,
    options: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    token_address: String,
    options: QueryParams,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountToken {
    address: String,
    token_address: String,
    options: QueryParams,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenId {
    token_address: String,
    id: u64,
    options: QueryParams,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockNumber {
    block: u64,
    options: QueryParams,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct QueryParams {
    chain: Option<String>,
}

pub fn check_query_params(params: &serde_json::Value) -> QueryParams {
    QueryParams {
        chain: Some(unescape(params["chain"].to_string().as_str()).unwrap()),
    }
}

pub async fn moralis_call(
    config: &Config,
    url: &String,
    params: QueryParams,
) -> Result<impl Responder, ApiError> {
    let awc_client = awc::Client::new();

    let response = awc_client
        .get(url)
        .insert_header(("x-api-key", config.moralis_api_key.to_owned()))
        .query(&params)
        .unwrap()
        .send()
        .await;

    match response {
        Ok(mut response) => {
            let body_str: String = std::str::from_utf8(&response.body().await.unwrap())
                .unwrap()
                .to_string();
            let body: web::Json<Value> = web::Json(serde_json::from_str(&body_str).unwrap());

            match response.status() {
                StatusCode::OK => Ok(body),
                _ => {
                    error!("Moralis API request failed: {}", body_str);
                    Err(ApiError::MoralisError)
                }
            }
        }
        Err(_) => Err(ApiError::MoralisError),
    }
}

pub async fn get_wallet_balance(
    req: web::Json<Address>,
    config: web::Data<Config>,
) -> Result<impl Responder, ApiError> {
    let url: String =
        config.moralis_base_url.to_owned() + &unescape(&req.address.as_str()).unwrap() + "/erc20";

    moralis_call(&config, &url, check_query_params(&req.options)).await
}

pub async fn get_wallet_transfers(
    req: web::Json<Address>,
    config: web::Data<Config>,
) -> Result<impl Responder, ApiError> {
    let url: String = config.moralis_base_url.to_owned()
        + &unescape(&req.address.as_str()).unwrap()
        + "/erc20/transfers";

    moralis_call(&config, &url, check_query_params(&req.options)).await
}

pub async fn get_contract_transfers(
    req: web::Json<Address>,
    config: web::Data<Config>,
) -> Result<impl Responder, ApiError> {
    let url: String = config.moralis_base_url.to_owned()
        + "/erc20"
        + &unescape(&req.address.as_str()).unwrap()
        + "/transfers";

    moralis_call(&config, &url, check_query_params(&req.options)).await
}
