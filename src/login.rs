// Used to generate SAML cookies and keys for use in further requests.
// Copyright (c) 2021 Kavika Palletenne

use hyper::{Client, Request, Body};
use hyper_tls::HttpsConnector;
use std::time::Instant;
use urlencoding::encode;

const MGS_SAML_LOGIN_ENTRYPOINT: &str = "https://my.mgs.vic.edu.au/mg/saml_login?destination=mymgs";
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn login() -> Result<String> {
    let now = Instant::now();

    let username = encode("kbpalletenne@student.mgs.vic.edu.au");
    let password = "12062004"; // TODO: Get these from ENV variables
    let (saml_post_url, simple_saml_session_id) = generate_saml_prerequisites().await?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    //////////////////////
    // Get MSISAUTH Cookie
    //////////////////////
    let body = format!("UserName={}&Password={}&AuthMethod=FormsAuthentication", username, password);

    let request = Request::builder()
        .method("POST")
        .uri(&saml_post_url)
        .header("Cookie", &simple_saml_session_id) // SimpleSAMLSessionID
        .header("Origin", "fs.mgs.vic.edu.au")
        .header("Referer", &saml_post_url)
        .body(Body::from(body))
        .unwrap();


    let response = client.request(request).await?;
    let msis_auth_cookie = &response.headers().get("Set-Cookie").unwrap().to_str().unwrap()[..1733];


    /////////////////////////////////////////////////////////////////
    // Get SamlSession, MSISAuthentcated & MSISLoopDetection Cookies
    /////////////////////////////////////////////////////////////////
    let next_saml_request_url = response.headers().get("Location").unwrap().to_str().unwrap();
    let cookie = format!("{}; {}", msis_auth_cookie, simple_saml_session_id);
    let request = Request::builder()
        .method("GET")
        .uri(next_saml_request_url)
        .header("Cookie", cookie) // SimpleSAMLSessionID
        .header("Origin", "fs.mgs.vic.edu.au")
        .header("Referer", &saml_post_url)
        .body(Body::empty())
        .unwrap();

    let response = client.request(request).await?;

    let response_cookies = response.headers().get_all("Set-Cookie");
    let mut iter = response_cookies.iter();

    let saml_session_cookie = &iter.next().unwrap().to_str().unwrap()[..344];
    let msis_authenticated_cookie = &iter.next().unwrap().to_str().unwrap()[..46];
    let msis_loop_detection_cookie = &iter.next().unwrap().to_str().unwrap()[..56];

    // TODO: The "SAML Response" needed for the next request is a "hidden" input field of the body of this response ^^^^^.
    let saml_response = // Get from the body of this response.

    // println!("{} {} {}", saml_session_cookie, msis_authenticated_cookie, msis_loop_detection_cookie);


    //////////////////////////
    // Get SimpleSAMLAuthToken
    //////////////////////////
    let body = format!("SAMLResponse={}&RelayState=https://my.mgs.vic.edu.au/mg/saml_login?destination=mymgs", saml_response);

    let request = Request::builder()
        .method("POST")
        .uri("https://my.mgs.vic.edu.au/simplesaml/module.php/sp/saml2-acs.php/default-sp")
        .header("Cookie", &simple_saml_session_id) // SimpleSAMLSessionID
        .header("Origin", "httsp://fs.mgs.vic.edu.au")
        .header("Referer", "https://fs.mgs.vic.edu.au/")
        .body(Body::from(body))
        .unwrap();

    let response = client.request(request).await?;
    let simple_saml_auth_token_cookie = &response.headers.get("Set-Cookie").unwrap().to_str().unwrap()[..]; //TODO: Find the length of the part of token I need
    // println!("authtoken: {}", response.headers().get("Set-Cookie").unwrap().to_str().unwrap());
    //TODO: These are all the responses I need, after that all I need to do is use these cookies when requesting the timetable and then parse the json and save to database.
    println!("Logged In as user {}: {}ms", username, now.elapsed().as_millis());
    Ok(msis_auth_cookie.to_string())

}

// Generate URL to push login info to + Generate SAML Session ID cookie
pub async fn generate_saml_prerequisites() -> Result<(String, String)> {
    let now = Instant::now();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let response = client.get(MGS_SAML_LOGIN_ENTRYPOINT.to_string().parse().unwrap()).await?;

    let saml_post_url = response.headers().get("Location").unwrap().to_str().unwrap(); // URL to POST login form with data
    let session_id = &response.headers().get("Set-Cookie").unwrap().to_str().unwrap()[..52]; // get only the session ID


    //println!("Generated SAML Prerequisites: {}ms", now.elapsed().as_millis());
    Ok((saml_post_url.to_string(), session_id.to_string()))
}