// Used to generate SAML cookies and keys for use in further requests.
// Copyright (c) 2021 Kavika Palletenne

use hyper::{Client, Request, Body};
use hyper_tls::HttpsConnector;
use std::time::Instant;
use url::form_urlencoded::Serializer;

const MGS_SAML_LOGIN_ENTRYPOINT: &str = "https://my.mgs.vic.edu.au/mg/saml_login?destination=mymgs";
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// TODO: Clean up this function
pub async fn login(username: &str, password: &str) -> Result<(String, String, String)> {
    let now = Instant::now();
    let (saml_post_url, simple_saml_session_id) = fetch_saml_prerequisites().await?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    //////////////////////
    // Get MSISAUTH Cookie
    //////////////////////
    let body: String = Serializer::new(String::new())
        .append_pair("UserName", username)
        .append_pair("Password", password)
        .append_pair("AuthMethod", "FormsAuthentication")
        .finish();

    let request = Request::builder()
        .method("POST")
        .uri(&saml_post_url)
        .header("Cookie", &simple_saml_session_id) // SimpleSAMLSessionID
        .body(Body::from(body))
        .unwrap();


    let response = client.request(request).await?;
    let msis_auth_cookie = &response.headers().get("Set-Cookie").unwrap().to_str().unwrap()[..1733];


    ///////////////////////
    // Get SAMLResponse Key
    ///////////////////////
    let next_saml_request_url = response.headers().get("Location").unwrap().to_str().unwrap();
    let cookie_header = format!("{}; {}", msis_auth_cookie, simple_saml_session_id);
    let request = Request::builder()
        .method("GET")
        .uri(next_saml_request_url)
        .header("Cookie", cookie_header) // SimpleSAMLSessionID
        .body(Body::empty())
        .unwrap();

    let response = client.request(request).await?;

    // Getting the SAMLResponseCookie from the response body.
    let body_bytes = hyper::body::to_bytes(response).await?;
    let body = String::from_utf8(body_bytes.to_vec()).expect("response was not valid utf-8");

    let saml_response = &body.to_string()[230..17442]; // Get from the body of the response.


    //////////////////////////
    // Get SimpleSAMLAuthToken
    //////////////////////////
    let body: String = Serializer::new(String::new())
        .append_pair("SAMLResponse", saml_response)
        .append_pair("RelayState", "https://my.mgs.vic.edu.au/mg/saml_login?destination=mymgs")
        .finish();

    let request = Request::builder()
        .method("POST")
        .uri("https://my.mgs.vic.edu.au/simplesaml/module.php/saml/sp/saml2-acs.php/default-sp")
        .header("Cookie", &simple_saml_session_id) // SimpleSAMLSessionID
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(body))
        .unwrap();

    let response = client.request(request).await?;
    let simple_saml_auth_token_cookie = &response.headers().get("Set-Cookie").unwrap().to_str().unwrap()[..63];


    ////////////////////////
    // Get SSESSxxxx Cookie
    ////////////////////////
    let cookie_header = format!("{}; {}", simple_saml_session_id, simple_saml_auth_token_cookie);
    let request = Request::builder()
        .method("GET")
        .uri("https://my.mgs.vic.edu.au/mg/saml_login?destination=mymgs")
        .header("Cookie", cookie_header)
        .body(Body::empty())
        .unwrap();

    let response = client.request(request).await?;

    let ssess_cookie = &response.headers().get("Set-Cookie").unwrap().to_str().unwrap()[..81];
    println!("Logged in: {}ms", now.elapsed().as_millis());
    Ok((simple_saml_session_id, simple_saml_auth_token_cookie.to_string(), ssess_cookie.to_string()))
}

// Generate URL to push login info to + Generate SAML Session ID cookie
pub async fn fetch_saml_prerequisites() -> Result<(String, String)> {
    let now = Instant::now();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let response = client.get(MGS_SAML_LOGIN_ENTRYPOINT.to_string().parse().unwrap()).await?;

    let saml_post_url = response.headers().get("Location").unwrap().to_str().unwrap(); // URL to POST login form with data
    let session_id = &response.headers().get("Set-Cookie").unwrap().to_str().unwrap()[..52]; // get only the session ID


    //println!("Fetched SAML Prerequisites: {}ms", now.elapsed().as_millis());
    Ok((saml_post_url.to_string(), session_id.to_string()))
}