use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Payload {
    pub catch_panics: bool,
    pub certificate_pinning_hosts: Value,
    pub custom_tls_client: Value,
    pub transport_options: Value,
    pub follow_redirects: bool,
    pub force_http1: bool,
    pub header_order: Vec<String>,
    pub headers: HashMap<String, String>,
    pub insecure_skip_verify: bool,
    pub is_byte_request: bool,
    pub is_byte_response: bool,
    pub is_rotating_proxy: bool,
    pub proxy_url: String,
    pub request_body: String,
    pub request_cookies: Value,
    pub default_headers: Value,
    pub request_method: String,
    pub request_url: String,
    #[serde(rename = "disableIPV6")]
    pub disable_ipv6: bool,
    pub local_address: Value,
    pub session_id: Value,
    pub server_name_overwrite: String,
    pub stream_output_block_size: Value,
    #[serde(rename = "streamOutputEOFSymbol")]
    pub stream_output_eofsymbol: Value,
    pub stream_output_path: Value,
    pub timeout_milliseconds: u128,
    pub timeout_seconds: i64,
    pub tls_client_identifier: String,
    pub with_debug: bool,
    pub with_default_cookie_jar: bool,
    pub without_cookie_jar: bool,
    #[serde(rename = "withRandomTLSExtensionOrder")]
    pub with_random_tlsextension_order: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Response {
    pub id: String,
    pub body: String,
    pub cookies: Value,
    pub headers: Value,
    pub status: i32,
    pub target: String,
    pub used_protocol: String,
}

impl Response {
    pub fn from_str(s: &str) -> Response {
        serde_json::from_str(s).unwrap()
    }
}

impl Payload {
    pub fn from_reqwest(b: reqwest::blocking::Request) -> Payload {
        let mut pl = Payload::default();

        pl.with_random_tlsextension_order = true;
        pl.tls_client_identifier = "chrome_120".to_string();

        pl.request_method = b.method().to_string();
        pl.request_url = b.url().to_string();
        if b.body().is_some() {
            pl.request_body = std::str::from_utf8(b.body().unwrap().as_bytes().unwrap())
                .unwrap()
                .to_string();
        }
        if b.timeout().is_some() {
            pl.timeout_milliseconds = b.timeout().unwrap().as_millis();
        }

        for h in b.headers() {
            pl.header_order.push(h.0.to_string());
            pl.headers
                .insert(h.0.to_string(), h.1.to_str().unwrap().to_string());
        }

        return pl;
    }
}
