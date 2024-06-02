use crate::extract_required::{lz_compress, ParsedScript};
use crate::traversals::config_builder::ChlData;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

static DOMAIN: &str = "cfschl.peet.ws";

pub fn get_page() -> Result<String, reqwest::Error> {
    let url = format!("https://{}/", DOMAIN);
    println!("GET {}", url);
    let resp = reqwest::blocking::get(url);
    resp?.text()
}

pub fn get_script(chl_data: &ChlData) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://{}/cdn-cgi/challenge-platform/h/{}/orchestrate/chl_page/v1?ray={}",
        DOMAIN, chl_data.c_fpwv, chl_data.c_ray
    );
    println!("GET {}", url);
    let resp = reqwest::blocking::get(url);
    resp?.text()
}

pub fn submit_init(
    chl_data: &ChlData,
    script_data: &ParsedScript,
) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let url = format!(
        "https://{}/cdn-cgi/challenge-platform/h/{}/flow/ov1/{}/{}/{}",
        DOMAIN, chl_data.c_fpwv, script_data.path, chl_data.c_ray, chl_data.c_hash
    );

    let key: &[u8] = "ATGU$-1bgpjyQfzIlDa0nuS3Fh5N2ixPR8CHc79eMOB+4XZmqWVEdr6kwvsoYtJKL".as_bytes();
    let payload = lz_compress(
        r#"{"wdfF9":"managed","nWwk5":"23165","XSdO4":"3","rkVkf2":0,"ypBGZ7":0,"hpNQ2":10.944999933242798,"wPAd0":4.8450000286102295,"wQbCqb2":1,"jUFldP6":{"ru":"aHR0cHM6Ly9jZnNjaGwucGVldC53cy8=","ra":"R28taHR0cC1jbGllbnQvMi4w","rm":"R0VU","d":"g+L3t7gu1UJEEiOVzKi2+dby/bK1ftB61AGErwQ3FopydjqhNCdLTvJuvnh7iyAkF/rxmXZBcV9SP0ieiyJmP64dHdoth7xeXgxRMt8Hvl/7ZItCv5xW9yl5FWhV+dZxmVviLUbrH3akUiR1jfg3ZvWh5iLTKPrKHcIMwllKLneTBFE1soSp0X1GQqRP8Q8AGwC7gafXyhZfD7WvL21jQoO+qQ21I0aIlLwYozl3EyOWdie38CDVufYwTVnN+Q3nbUDvbZ86HFUzKqDH/eva/btdBqamx9w+KcKuA8Utw3J0aBpWx8irKt4Y46Y618c/ZXX7qgRI0jWMQKMUAZoVh7kIiPzl38irDJygCGRMNrwrewuYAMXLzyxyo9cnb1Wk2YOmPDO6JcfuHSOVY4CsRJHfJJH+W/uhOa78i8mmIExpHmHB5Frd8XcUySZDv1VJf3AJ+2I7XTK0ojqR4Y8CTO+k6bnbx56tCe1/BFarjNDpado+HIvn7K+0R9vYfP1h851b8GFfWhL364oKc2204+orGVFQYSWAwX7IdnmZUhDB0G5HY7HOO952zfdYHZqMAMB7tt8F9aU8qK5nCjWowQ==","t":"MTcxNDkyMjI1MC4zMTAwMDA=","cT":1714922250,"m":"Z0CPUPoXW71jcaoWJDby1fhsY+8Tk4acwGNXgWKycnA=","i1":"8S0UfES9oGbBqpFs4U/H6A==","i2":"y/rvHY86aAMamcs9Tc2h+g==","zh":"g0oLPmGBHOOANgRh/0LsbsmhK+eLKswnGfZ3vKZJJPk=","uh":"AN+5vvENMpdOACvuRa1plLx7uNq9y3wYG+zurIv4jHM=","hh":"9SXEBZ7jq2GTlkpWURqPC0QcqE9KkBpV1ezCnFXR0aY="},"gKrrTp6":{"MOtpN1":0,"BUlIns2":0,"jEDt8":0,"mVIa4":0,"ItzCLb5":0,"WTCWQa7":0,"wnncq8":0,"YVriY2":0},"GpeX6":false,"BJbOsT8":0})"#,
        key,
    );

    // println!("{}", payload);

    let body = format!("v_{}={}", chl_data.c_ray, payload);

    println!("POST {}", url);
    let mut headers = HeaderMap::new();
    headers.insert(
        "content-type",
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    // headers.insert("cf-challenge", HeaderValue::from_static());
    let resp = client.post(url).body(body).headers(headers).send();
    // let resp = reqwest::blocking::post(url);
    resp?.text()
}
