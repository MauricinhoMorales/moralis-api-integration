use crate::{error::ApiError, config::Config};
use actix_web::{http::StatusCode, web, Responder};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use log::error;
use snailquote::unescape;

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    address: String,
    options: QueryParams
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    token_address: String,
    options: QueryParams
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountToken {
    address: String,
    token_address: String,
    options: QueryParams
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenId {
    token_address: String,
    id: u64,
    options: QueryParams
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockNumber {
    block: u64,
    options: QueryParams
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct QueryParams {
    chain: Option<String>,
    format: Option<String>,
    offset: Option<u64>,
    limit: Option<u64>
}

pub fn check_query_params(params: &QueryParams) -> QueryParams {

    QueryParams {
        chain: match &params.chain {
            Some(chain) => Some(chain.to_string()),
            None => Some("ropsten".to_string()),
        },
        format: match &params.format {
            Some(format) => Some(format.to_string()),
            None => Some("decimal".to_string()),
        },
        offset: match &params.offset {
            Some(offset) => Some(*offset),
            None => Some(0),
        },
        limit: match &params.limit {
            Some(limit) => Some(*limit),
            None => Some(10),
        },
    }
}

pub async fn moralis_call(config: &Config, url: &String, params: QueryParams) -> Result<impl Responder, ApiError> {

    let awc_client = awc::Client::new();

    let response = 
        awc_client.get(url)
            .insert_header(("x-api-key", config.moralis_api_key.to_owned()))
            .query(&params).unwrap()
            .send()
            .await;

    match response {
        Ok(mut response) => {
            let body_str: String = std::str::from_utf8(&response.body().await.unwrap()).unwrap().to_string();
            let body: web::Json<Value> = web::Json(serde_json::from_str(&body_str).unwrap());
            
            match response.status() {
                StatusCode::OK => Ok(body),
                _ => {
                    error!("Moralis API request failed: {}", body_str);
                    Err(ApiError::MoralisError)
                }
            }
        },
        Err(_) => Err(ApiError::MoralisError)
    }
}

pub async fn get_balance(_req_body: web::Json<Address>, config: web::Data<Config>) -> Result<impl Responder, ApiError> {
    let address = "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b";
    let options = QueryParams {
        chain: Some("goerli".to_owned()),
        offset: None,
        format: None,
        limit: None
    };

    let url: String = config.moralis_base_url.to_owned() + &unescape(address).unwrap() + "/erc20";

    moralis_call(&config, &url, check_query_params(&options)).await
}
