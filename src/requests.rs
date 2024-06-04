use crate::extract_required::{lz_compress, ParsedScript};
use crate::tls_api;
use crate::traversals;
use reqwest::blocking::Client;
use reqwest::header::HeaderValue;

static DOMAIN: &str = "cfschl.peet.ws";

struct Device<'a> {
    sec_ch_ua: &'a str,
    sec_ch_ua_mobile: &'a str,
    user_agent: &'a str,
    sec_ch_ua_platform: &'a str,
    language: &'a str,
    sec_ch_ua_full_version: &'a str,
    sec_ch_ua_arch: &'a str,
    sec_ch_ua_full_version_list: &'a str,
}

impl Device<'_> {
    pub fn brave() -> Device<'static> {
        return Device {
            sec_ch_ua: "\"Google Chrome\";v=\"125\", \"Chromium\";v=\"125\", \"Not.A/Brand\";v=\"24\"",
            sec_ch_ua_mobile: "?0",
            sec_ch_ua_platform: "\"macOS\"",
            sec_ch_ua_full_version: "\"125.0.6422.113\"",
            sec_ch_ua_arch: "\"arm\"",
            sec_ch_ua_full_version_list: "\"Google Chrome\";v=\"125.0.6422.113\", \"Chromium\";v=\"125.0.6422.113\", \"Not.A/Brand\";v=\"24.0.0.0\"",
            user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
            language: "en-DE,en-US;q=0.9,en;q=0.8",
        };
    }
}

fn sh(s: &str) -> HeaderValue {
    return HeaderValue::from_str(s).unwrap();
}

pub struct SolvingSession<'a> {
    pub cnfg: traversals::config_builder::VMConfig,
    pub domain: String,
    pub debug: bool,

    client: Client,
    device: Device<'a>,
    tls_api: bool,
}

impl SolvingSession<'_> {
    pub fn new(domain: &str, debug: bool) -> SolvingSession {
        let tls_api = false;
        let tmp = Client::builder()
            .http1_title_case_headers()
            .brotli(true)
            .danger_accept_invalid_certs(true);
        let c: Client;
        if debug && !tls_api {
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
            device: Device::brave(),
            client: c,
            debug,
            tls_api,
        };
    }

    fn use_tls_client_api(&self, r: reqwest::blocking::RequestBuilder) -> String {
        let b = r.build().unwrap();

        let mut pl = tls_api::Payload::from_reqwest(b);

        if self.debug {
            pl.proxy_url = "http://localhost:8888".to_string();
        }

        let j = serde_json::to_string_pretty(&pl).unwrap();

        let res = self
            .client
            .post("http://localhost:9999/api/forward")
            .header("x-api-key", "swccf")
            .header("Content-Type", "application/json")
            .body(j)
            .send();

        let parsed = tls_api::Response::from_str(res.unwrap().text().unwrap().as_str());

        return parsed.body;
    }

    pub fn get_page(&self) -> Result<String, reqwest::Error> {
        let url = format!("https://{}/", DOMAIN);
        let req = self
        .client
        .get(url)
        .header("sec-ch-ua", sh(self.device.sec_ch_ua))
        .header("sec-ch-ua-mobile", sh(self.device.sec_ch_ua_mobile))
        .header("sec-ch-ua-full-version", sh(self.device.sec_ch_ua_full_version))
        .header("sec-ch-ua-arch", sh(self.device.sec_ch_ua_arch))
        .header("sec-ch-ua-platform", sh(self.device.sec_ch_ua_platform))
        .header("sec-ch-ua-platform-version", sh("\"14.3.0\""))
        .header("sec-ch-ua-model", sh("\"\""))
        .header("sec-ch-ua-bitness", sh("\"64\""))
        .header("sec-ch-ua-full-version-list", sh(self.device.sec_ch_ua_full_version_list))
        .header("dnt", "1")
        .header("upgrade-insecure-requests", "1")
        .header("user-agent", sh(self.device.user_agent))
        .header("accept", sh("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"))
        .header("sec-fetch-site", sh("none"))
        .header("sec-fetch-mode", sh("navigate"))
        .header("sec-fetch-user", sh("?1"))
        .header("sec-fetch-dest", sh("document"))
        .header("accept-encoding", sh("gzip, deflate, br, zstd"))
        .header("accept-language", sh(self.device.language))
        .header("priority", sh("u=0, i"));

        if self.tls_api {
            return Ok(self.use_tls_client_api(req));
        } else {
            return req.send()?.text();
        }
    }

    pub fn get_script(&self) -> Result<String, reqwest::Error> {
        let url = format!(
            "https://{}/cdn-cgi/challenge-platform/h/{}/orchestrate/chl_page/v1?ray={}",
            self.domain, self.cnfg.chl_data.c_fpwv, self.cnfg.chl_data.c_ray
        );
        let referer = &format!("https://cfschl.peet.ws{}", self.cnfg.chl_data.fa);
        let req = self
            .client
            .get(url)
            .header("sec-ch-ua", sh(self.device.sec_ch_ua))
            .header("dnt", "1")
            .header("sec-ch-ua-mobile", sh(self.device.sec_ch_ua_mobile))
            .header("user-agent", sh(self.device.user_agent))
            .header("sec-ch-ua-arch", sh(self.device.sec_ch_ua_arch))
            .header(
                "sec-ch-ua-full-version",
                sh(self.device.sec_ch_ua_full_version),
            )
            .header("sec-ch-ua-platform-version", sh("\"14.3.0\""))
            .header(
                "sec-ch-ua-full-version-list",
                sh(self.device.sec_ch_ua_full_version_list),
            )
            .header("sec-ch-ua-bitness", sh("\"64\""))
            .header("sec-ch-ua-model", sh("\"\""))
            .header("sec-ch-ua-platform", sh(self.device.sec_ch_ua_platform))
            .header("accept", sh("*/*"))
            .header("sec-fetch-site", sh("same-origin"))
            .header("sec-fetch-mode", sh("no-cors"))
            .header("sec-fetch-dest", sh("script"))
            .header("referer", sh(referer))
            .header("accept-encoding", sh("gzip, deflate, br, zstd"))
            .header("accept-language", sh(self.device.language));

        if self.tls_api {
            return Ok(self.use_tls_client_api(req));
        } else {
            return req.send()?.text();
        }
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

        let req = self
            .client
            .post(url)
            .header("content-length", sh(&format!("{}", body.len())))
            .header("sec-ch-ua", sh(self.device.sec_ch_ua))
            .header("dnt", "1")
            .header("sec-ch-ua-mobile", sh(self.device.sec_ch_ua_mobile))
            .header("user-agent", sh(self.device.user_agent))
            .header("sec-ch-ua-arch", sh(self.device.sec_ch_ua_arch))
            .header("content-type", sh("application/x-www-form-urlencoded"))
            .header(
                "sec-ch-ua-full-version",
                sh(self.device.sec_ch_ua_full_version),
            )
            .header("sec-ch-ua-platform-version", sh("\"14.3.0\""))
            .header(
                "sec-ch-ua-full-version-list",
                sh(self.device.sec_ch_ua_full_version_list),
            )
            .header("sec-ch-ua-bitness", sh("\"64\""))
            .header("sec-ch-ua-model", sh("\"\""))
            .header("cf-challenge", sh(&self.cnfg.chl_data.c_hash))
            .header("sec-ch-ua-platform", sh(self.device.sec_ch_ua_platform))
            .header("accept", sh("*/*"))
            .header("origin", sh("https://cfschl.peet.ws"))
            .header("sec-fetch-site", sh("same-origin"))
            .header("sec-fetch-mode", sh("cors"))
            .header("sec-fetch-dest", sh("empty"))
            .header("referer", sh("https://cfschl.peet.ws/"))
            .header("accept-encoding", sh("gzip, deflate, br, zstd"))
            .header("accept-language", sh(self.device.language))
            .header("priority", sh("u=1, i"))
            .body(body);

        if self.tls_api {
            return Ok(self.use_tls_client_api(req));
        } else {
            return req.send()?.text();
        }
    }
}
