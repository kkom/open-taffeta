extern crate open_taffeta_lib;

extern crate reqwest;

#[macro_use]
extern crate serde_json;

use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::{Client, StatusCode};

mod common;

use crate::common::dbstate::DbState;

// TODO: have a look here
// https://bitbucket.org/dorianpula/rookeries/src/master/tests/test_site_management.rs

#[test]
fn test_bad_auth() {
    DbState::new();
    let api_base_uri = common::api_base_url();
    let client = Client::new();
    let payload = json!({"name":"door123"});
    let mut response = client
        .post(api_base_uri.join("/door").unwrap())
        .json(&payload)
        .header(AUTHORIZATION, HeaderValue::from_static("hahaha"))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let resp_str: &str = &response.text().unwrap().to_string();
    assert_eq!(resp_str.contains("Not authorized"), true);

}

#[test]
fn test_create() {
    let state = DbState::new();
    let api_base_uri = common::api_base_url();
    let (_, token) = common::signup_user("josh", "josh@domain.com");
    let client = Client::new();

    // check for 0 doors
    state.assert_empty_doors();

    let payload = json!({"name":"door123"});
    let response = client
        .post(api_base_uri.join("/door").unwrap())
        .json(&payload)
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // check new door
    let response = client
        .get(api_base_uri.join("/doors").unwrap())
        .json(&payload)
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn test_delete() {
    let state = DbState::new();
    let api_base_uri = common::api_base_url();
    let (_, token) = common::signup_user("josh", "josh@domain.com");
    let client = Client::new();

    // check for 0 doors
    state.assert_empty_doors();

    let payload = json!({"name":"door123"});
    let mut response = client
        .post(api_base_uri.join("/door").unwrap())
        .json(&payload)
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let door_data : common::ResponseDoorCreated = response.json().unwrap();

    // check new door
    let url = &format!("/door/{}", door_data.door.id);
    let response = client
        .delete(api_base_uri.join(url).unwrap())
        .json(&payload)
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}