use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};

const SESSION_COOKIE_ENV: &'static str = "SESSION_COOKIE";

pub async fn input(day: u8) -> Box<str> {
    let mut headers = HeaderMap::new();
    let session_cookie =
        std::env::var(SESSION_COOKIE_ENV).expect(&format!("{SESSION_COOKIE_ENV} not set")); //""
    headers.append(
        header::COOKIE,
        HeaderValue::from_str(&format!("session={session_cookie}"))
            .expect("invalid_session_cookie"),
    );
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to create the client!");
    return client
        .get(format!("https://adventofcode.com/2024/day/{day}/input"))
        .send()
        .await
        .expect("Failed to send request")
        .text()
        .await
        .expect("Failed to parse request")
        .into_boxed_str();
}
