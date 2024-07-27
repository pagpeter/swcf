use crate::utils::utils;
use core::fmt;
use rand::Rng;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagicBits {
    pub start_enc: u64,
    pub opcode_enc: u64,
    pub enc: Vec<u64>,

    pub bind_func: Vec<u64>,
    pub shuffle_reg: Vec<u64>,
    pub binary_exp: Vec<u64>,
    pub unary_exp: Vec<u64>,
    pub new_arr: Vec<u64>,
    pub jump: Vec<u64>,
    pub jump_if: Vec<u64>,
    pub get_obj: Vec<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CRq {
    pub ru: String,
    pub ra: String,
    pub rm: String,
    pub d: String,
    pub t: String,
    #[serde(skip_deserializing)]
    pub c_t: u64,
    pub m: String,
    pub i1: String,
    pub i2: String,
    pub zh: String,
    pub uh: String,
    pub hh: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChlData {
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
    #[serde(skip_deserializing)]
    pub c_tpl_v: i32,
    pub c_tpl_b: String,
    pub c_k: String,
    pub fa: String,
    pub md: String,
    pub mdrd: String,
    pub c_rq: CRq,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bytecodes {
    pub init: String,
    pub main: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payloads {
    pub init: String,
    pub main: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VMConfig {
    pub payloads: Payloads,
    pub registers: HashMap<String, u64>,
    pub magic_bits: MagicBits,
    pub bytecodes: Bytecodes,
    pub chl_data: ChlData,
}

impl VMConfig {
    fn find_start_enc(&mut self, script: &str) {
        let caps = utils::find_from_multiple_regexes(
            script,
            vec![r"atob\(.\),(\d+)", r"atob,.\),(\d+?),"],
        );
        if caps.is_none() {
            println!("[!] Could not get start enc")
        } else {
            self.magic_bits.start_enc = caps.unwrap()[1].parse().unwrap();
        }
    }
    fn find_opcode_enc(&mut self, script: &str) {
        let caps = utils::find_from_multiple_regexes(script, vec![r"\+\+\)-(\d{1,}),256"]);
        if caps.is_none() {
            println!("[!] Could not opcode enc")
        } else {
            self.magic_bits.opcode_enc = caps.unwrap()[1].parse().unwrap();
        }
    }
    pub fn find_all_enc(&mut self, script: &str) {
        self.find_opcode_enc(script);
        self.find_start_enc(script)
    }
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    ArrPop,
    ArrPush,
    SetMem,
    Apply,
    NewArr,
    JumpIf,
    GetObj,
    SetObj,
    SplicePop,
    BindFunc,
    BindFunc2,
    Jump,
    NewClass,
    NewObj,
    ThrowError,
    ShuffleReg,
    UnaryExp,
    BinaryExp,
    Literal,
    WeirdNew,
    Invalid,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayloadKey {
    pub key: String,
    pub value_type: String,
    pub num_value: f64,
    pub data_key: String,
    pub str_value: String,
    pub sub_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct InitKeys {
    pub keys: Vec<PayloadKey>,
}

impl InitKeys {
    pub fn insert_in_place(&mut self, value: PayloadKey, index: usize) {
        self.keys[index..].rotate_right(1);
        self.keys[index] = value;
    }

    // Marshals the init payload - dynamic keys from the script
    pub fn marshal(&self, cnfg: &VMConfig) -> String {
        let mut j: String = "{".to_owned();
        for k in &self.keys {
            if k.value_type == "NUMBER" {
                j += &format!("\"{}\":{},", k.key, k.num_value.round())
            } else if k.value_type == "STRING" {
                j += &format!("\"{}\":\"{}\",", k.key, k.str_value)
            } else if k.value_type == "RANDOM" {
                j += &format!("\"{}\":{},", k.key, rand::thread_rng().gen_range(1..20))
            } else if k.value_type == "SENSOR" {
                j += &format!("\"{}\":{{", k.key);
                for sub in &k.sub_keys {
                    j += &format!("\"{}\":0,", sub)
                }
                j += &format!("}},");
            } else if k.value_type == "DATA" {
                let val: String;

                if k.data_key == "cType" {
                    val = format!("\"{}\"", cnfg.chl_data.c_type.to_string());
                } else if k.data_key == "cNounce" {
                    val = format!("\"{}\"", cnfg.chl_data.c_nounce.to_string());
                } else if k.data_key == "cvId" {
                    val = format!("\"{}\"", cnfg.chl_data.cv_id.to_string());
                } else if k.data_key == "cRq" {
                    val = serde_json::to_string(&cnfg.chl_data.c_rq).unwrap();
                } else {
                    // println!("Not implemented: {}", k.data_key);
                    val = "false".to_owned();
                }
                j += &format!("\"{}\":{},", k.key, val);
            }
        }
        j += "}";
        j = j.replace(",}", "}");
        return j;
    }
}
