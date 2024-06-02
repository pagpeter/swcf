use core::fmt::Error;
use regex::Regex;
use serde_json;

use crate::traversals::config_builder::ChlData;
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

pub fn parse_challenge_data(result: &str) -> Result<ChlData, Error> {
    let re = Regex::new(r"window._cf_chl_opt=(.+?);var").unwrap();
    let Some(caps) = re.captures(&result) else {
        println!("Error regexing _cf_chl_opt");
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

    let parsed: Result<ChlData, serde_json::Error> = serde_json::from_str(&res2);

    if parsed.is_err() {
        println!("Error marshalling _cf_chl_opt as json");
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
