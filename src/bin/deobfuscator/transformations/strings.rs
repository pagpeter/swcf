use regex::Regex;
use swc_core::ecma::atoms::JsWord;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{Expr, Lit, Program, Str};
use swc_ecma_visit::{Visit, VisitWith};
// use swc_ecma_visit::{VisitAllWith, VisitWith};

#[derive(Default)]
struct FindInteger {
    ints: Vec<f64>,
}
impl Visit for FindInteger {
    fn visit_number(&mut self, n: &swc_ecma_ast::Number) {
        self.ints.push(n.value)
    }
}

#[derive(Default)]

struct ReplaceProxyCalls {
    subtract: i32,
    strings: Vec<String>,
}

impl ReplaceProxyCalls {
    pub fn new(subtract: i32, strings: Vec<String>) -> Self {
        Self { subtract, strings }
    }
}

impl VisitMut for ReplaceProxyCalls {
    fn visit_mut_expr(&mut self, expr: &mut swc_ecma_ast::Expr) {
        expr.visit_mut_children_with(self);

        if !expr.is_call() {
            return;
        }
        let n = expr.as_call().unwrap();

        if n.args.len() != 1 {
            return;
        }

        let arg = n.args[0].expr.as_lit();
        if let Some(p) = arg {
            let mut find = FindInteger::default();
            p.to_owned().visit_children_with(&mut find);
            if find.ints.len() == 1 {
                let i: i32 = find.ints[0] as i32;

                let works = usize::try_from(i - self.subtract);
                if let Ok(res) = works {
                    let str = self.strings[res].to_owned();
                    *expr = Expr::Lit(Lit::Str(Str::from(str)));
                }
            }
        }
    }
}

#[derive(Default)]
struct FindAllStrings {
    done_string: bool,
    done_json: bool,
    strings: Vec<String>,
    json_start: u32,
}

impl VisitMut for FindAllStrings {
    fn visit_mut_atom(&mut self, n: &mut JsWord) {
        if self.done_string {
            return;
        }
        let length = n.len();

        if length > 200 {
            let re = Regex::new(r"bigint(?<delimiter>.)").unwrap();
            let all: String = n.to_string();
            if let Some(caps) = re.captures(&all) {
                self.done_string = true;
                let delimiter = &caps["delimiter"];
                self.strings = all.split(delimiter).map(String::from).collect();
                *n = JsWord::new("");
            }
        }
    }
    fn visit_mut_ident(&mut self, n: &mut swc_ecma_ast::Ident) {
        if n.sym != "JSON" || self.done_json {
            return;
        }
        self.json_start = n.span.lo.0 + 6;
        self.done_json = true;
    }
}

pub struct Visitor {
    source: String,
    stringify: i32,
    subtract: i32,
}

impl Visitor {
    pub fn new(source: String) -> Self {
        Self {
            source,
            stringify: 0,
            subtract: 0,
        }
    }
}

impl VisitMut for Visitor {
    fn visit_mut_program(&mut self, program: &mut Program) {
        println!("[*] Finding string array");
        let mut obf_strings = FindAllStrings::default();
        program.visit_mut_children_with(&mut obf_strings);
        // println!("{} {}", obf_strings.done_string, obf_strings.json_start);
        let splits = self
            .source
            .split_at(obf_strings.json_start.try_into().unwrap())
            .1
            .split_at(20)
            .0;

        let first_int_re = Regex::new(r"(?<int>\d+)").unwrap();
        if let Some(caps) = first_int_re.captures(splits) {
            self.stringify = caps["int"].parse::<i32>().unwrap();
        }
        let subtract_re = Regex::new(r".=.-(?<subtract>\d+?),.=").unwrap();
        if let Some(caps) = subtract_re.captures(&self.source) {
            self.subtract = caps["subtract"].parse::<i32>().unwrap();
        }
        // println!("stringify: {}, subtract: {}", self.stringify, self.subtract);
        loop {
            obf_strings.strings.rotate_left(1);
            if obf_strings.strings[usize::try_from(self.stringify - self.subtract).unwrap()]
                == "stringify"
            {
                break;
            }
        }
        println!("[*] Replacing string proxies");
        let mut replacer = ReplaceProxyCalls::new(self.subtract, obf_strings.strings);
        program.visit_mut_children_with(&mut replacer);
    }
}
