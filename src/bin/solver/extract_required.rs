use core::fmt::Error;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
pub struct ParsedScript {
    pub seperator: String,
    pub key: String,
    pub path: String,
}

pub fn parse_script(script: &str) -> ParsedScript {
    let seperator_regex = Regex::new(r"en-us(.)").unwrap();
    let seperator: String = seperator_regex.captures(&script).unwrap()[1].to_string();

    let key_regex = Regex::new(&format!(
        r"{}([a-zA-Z0-9\+\-\$]{{65}}){}",
        seperator, seperator
    ))
    .unwrap();
    let path_regex = Regex::new(r"/(\d+:\d+:.+?)/").unwrap();

    let key: String = key_regex.captures(&script).unwrap()[1].to_string();
    let path: String = path_regex.captures(&script).unwrap()[1].to_string();

    ParsedScript {
        seperator,
        key,
        path,
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeData {
    pub cv_id: String,
    pub c_zone: String,
    pub c_type: String,
    pub c_nounce: String,
    pub c_ray: String,
    pub c_hash: String,
    #[serde(rename = "cUPMDTk")]
    pub c_upmdtk: String,
    #[serde(rename = "cFPWv")]
    pub c_fpwv: String,
    #[serde(rename = "cTTimeMs")]
    pub c_ttime_ms: String,
    #[serde(rename = "cMTimeMs")]
    pub c_mtime_ms: String,
    pub c_tpl_v: String,
    pub c_tpl_b: String,
    pub c_k: String,
    pub fa: String,
    pub md: String,
    pub mdrd: String,
    pub c_rq: CRq,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CRq {
    pub ru: String,
    pub ra: String,
    pub rm: String,
    pub d: String,
    pub t: String,
    pub c_t: String,
    pub m: String,
    pub i1: String,
    pub i2: String,
    pub zh: String,
    pub uh: String,
    pub hh: String,
}

pub fn parse_challenge_data(result: &str) -> Result<ChallengeData, Error> {
    let re = Regex::new(r"window._cf_chl_opt=(.+?);var").unwrap();
    let Some(caps) = re.captures(&result) else {
        return Err(Error {});
    };

    let raw_object = caps[1].replace("'", "\"").replace("\"", "");

    let re2 = Regex::new(r"([\{\s,])(\w+)(:)").unwrap();
    let res2: String = re2
        .replace_all(&raw_object, "\"${1}\"${2}\"${3}\"")
        .replace(":\" ", ":\"")
        .replace(",}}", "\"}}")
        .replace("\"{", "{")
        .replace("\"{\"", "{\"");

    let parsed: Result<ChallengeData, serde_json::Error> = serde_json::from_str(&res2);

    if parsed.is_err() {
        return Err(Error {});
    }

    return Ok(parsed.unwrap());
}

pub fn lz_compress(data: impl lz_str::IntoWideIter, key: &[u8]) -> String {
    let data: Vec<u16> = data.into_wide_iter().collect();
    let mut compressed = lz_str::compress_internal(&data, 6, |n| u16::from(key[usize::from(n)]));

    let mod_4 = compressed.len() % 4;

    if mod_4 != 0 {
        for _ in mod_4..(4 + 1) {
            compressed.push(u16::from(b'='));
        }
    }

    String::from_utf16(&compressed).expect("`compress_to_base64` output was not valid unicode`")
}
