use crate::extract_required::{lz_compress, ParsedScript};
use crate::traversals;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

static DOMAIN: &str = "cfschl.peet.ws";

fn get_headers(c_ray: &str, c_len: usize) -> HeaderMap {
    let mut h = HeaderMap::new();

    fn sh(s: &str) -> HeaderValue {
        return HeaderValue::from_str(s).unwrap();
    }

    if c_len != 0 {
        h.insert("content-length", sh(&format!("{}", c_len)));
    }

    h.insert(
        "sec-ch-ua",
        sh("\"Chromium\";v=\"124\", \"Brave\";v=\"124\", \"Not-A.Brand\";v=\"99\""),
    );
    h.insert("sec-ch-ua-mobile", sh("?0"));
    h.insert("user-agent", sh("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"));
    h.insert("content-type", sh("application/x-www-form-urlencoded"));
    h.insert("sec-ch-ua-platform-version", sh("\"14.3.0\""));
    h.insert("sec-ch-ua-model", sh("\"\""));

    if c_ray.len() > 0 {
        h.insert("cf-challenge", HeaderValue::from_str(c_ray).unwrap());
    }

    h.insert("sec-ch-ua-platform", sh("\"macOS\""));
    h.insert("accept", sh("*/*"));
    h.insert("sec-gpc", sh("1"));
    h.insert("accept-language", sh("de-DE,de;q=0.7"));
    h.insert("origin", sh("https://cfschl.peet.ws"));
    h.insert("sec-fetch-site", sh("same-origin"));
    h.insert("sec-fetch-mode", sh("cors"));
    h.insert("sec-fetch-dest", sh("empty"));
    h.insert("referer", sh("https://cfschl.peet.ws/"));
    h.insert("accept-encoding", sh("gzip, deflate, br, zstd"));
    h.insert("priority", sh("u=1, i"));

    return h;
}

pub struct SolvingSession {
    pub cnfg: traversals::config_builder::VMConfig,
    pub domain: String,
    client: Client,
    pub debug: bool,
}

impl SolvingSession {
    pub fn new(domain: &str, debug: bool) -> SolvingSession {
        let tmp = Client::builder()
            .http1_title_case_headers()
            .brotli(true)
            .danger_accept_invalid_certs(true);
        let c: Client;
        if debug {
            println!("[DEBUG] Using debug proxy");
            c = tmp
                .proxy(reqwest::Proxy::all("http://localhost:8888").unwrap())
                .build()
                .unwrap();
        } else {
            c = tmp.build().unwrap()
        }

        return SolvingSession {
            domain: domain.to_owned(),
            cnfg: traversals::config_builder::VMConfig::default(),
            client: c,
            debug,
        };
    }
    pub fn get_page(&self) -> Result<String, reqwest::Error> {
        let url = format!("https://{}/", DOMAIN);
        println!("GET {}", url);
        let resp = self.client.get(url).headers(get_headers("", 0)).send();
        resp?.text()
    }

    pub fn get_script(&self) -> Result<String, reqwest::Error> {
        let url = format!(
            "https://{}/cdn-cgi/challenge-platform/h/{}/orchestrate/chl_page/v1?ray={}",
            self.domain, self.cnfg.chl_data.c_fpwv, self.cnfg.chl_data.c_ray
        );
        println!("GET {}", url);
        let resp = self.client.get(url).headers(get_headers("", 0)).send();
        resp?.text()
    }

    pub fn submit_init(&self, script_data: &ParsedScript) -> Result<String, reqwest::Error> {
        let url = format!(
            "https://{}/cdn-cgi/challenge-platform/h/{}/flow/ov1/{}/{}/{}",
            self.domain,
            self.cnfg.chl_data.c_fpwv,
            script_data.path,
            self.cnfg.chl_data.c_ray,
            self.cnfg.chl_data.c_hash
        );

        let key: &[u8] = script_data.key.as_bytes();
        let payload = lz_compress(&self.cnfg.payloads.init, key);

        let c_ray = &self.cnfg.chl_data.c_ray;

        let body = format!("v_{}={}", c_ray, payload.replacen("+", "%2b", 1));

        // println!("POST {}", url);
        // println!("KEY: {}", script_data.key);
        // println!("{}", body);

        let resp = self
            .client
            .post(url)
            .headers(get_headers(&self.cnfg.chl_data.c_hash, body.len()))
            .body(body)
            .send();
        resp?.text()
    }
}
