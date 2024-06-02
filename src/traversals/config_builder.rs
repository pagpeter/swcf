use core::fmt;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagicBits {
    start_enc: i32,
    opcode_enc: Vec<i32>,
    // NEW_ARR: Vec<i32>,
    // JUMP_IF: Vec<i32>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VMConfig {
    pub payloads: Payloads,
    pub registers: HashMap<String, f64>,
    pub magic_bits: MagicBits,
    pub bytecodes: Bytecodes,
    pub chl_data: ChlData,
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

#[derive(Default, Debug, Clone)]
pub struct PayloadKey {
    pub key: String,
    pub value_type: String,
    pub num_value: f64,
    pub data_key: String,
    pub sub_keys: Vec<String>,
}
