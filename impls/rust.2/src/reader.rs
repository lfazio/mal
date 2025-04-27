use std::collections::HashMap;
use std::rc::Rc;
use std::vec::Vec;

use regex::Regex;
use rustyline::DefaultEditor;

use crate::types::MalReturn;
use crate::types::MalVal;
use crate::types::error::MalError;

// macro to create a list
use crate::list;

struct Reader {
    tokens: Vec<String>,
    pos: usize,
}

impl Reader {
    fn peek(&self) -> Result<&str, MalError> {
        Ok(self
            .tokens
            .get(self.pos)
            .ok_or_else(|| MalError::Error("underflow".to_string()))?)
    }

    fn next(&mut self) -> Result<&str, MalError> {
        self.pos += 1;
        Ok(self
            .tokens
            .get(self.pos - 1)
            .ok_or_else(|| MalError::Error("underflow".to_string()))?)
    }

    fn skip(&mut self) {
        self.pos += 1;
    }
}

pub fn read(rl: &mut DefaultEditor, prompt: &str) -> MalReturn {
    match rl.readline(prompt) {
        Ok(line) => {
            let _ = rl.add_history_entry(line.as_str());

            read_str(&line)
        }
        Err(rustyline::error::ReadlineError::Interrupted) => {
            Err(MalError::Break("CTRL-C".to_string()))
        }
        Err(rustyline::error::ReadlineError::Eof) => Err(MalError::Break("CTRL-D".to_string())),
        Err(e) => Err(MalError::Readline(format!("{}", e))),
    }
}

pub fn read_str(line: &str) -> MalReturn {
    let tokens = tokenise(line)?;

    if tokens.is_empty() {
        return Err(MalError::Error("no input".to_string()));
    }

    read_form(&mut Reader { tokens, pos: 0 })
}

fn tokenise(line: &str) -> Result<Vec<String>, MalError> {
    let re =
        Regex::new(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###)
            .unwrap();

    let mut tokens: Vec<String> = vec![];

    for cap in re.captures_iter(line) {
        if cap[1].starts_with(";") {
            continue;
        }

        tokens.push(String::from(&cap[1]));
    }

    Ok(tokens)
}

fn unescape_str(s: &str) -> String {
    let re = Regex::new(r#"\\(.)"#).unwrap();

    re.replace_all(s, |caps: &regex::Captures| {
        if &caps[1] == "n" { "\n" } else { &caps[1] }.to_string()
    })
    .to_string()
}

fn read_form(rdr: &mut Reader) -> MalReturn {
    let token = rdr.peek()?;
    match token {
        "'" => {
            rdr.skip();
            Ok(list!(vec![
                MalVal::Symbol("quote".to_string()),
                read_form(rdr)?,
            ]))
        }
        "`" => {
            rdr.skip();
            Ok(list!(vec![
                MalVal::Symbol("quasiquote".to_string()),
                read_form(rdr)?,
            ]))
        }
        "~" => {
            rdr.skip();
            Ok(list!(vec![
                MalVal::Symbol("unquote".to_string()),
                read_form(rdr)?,
            ]))
        }
        "~@" => {
            rdr.skip();
            Ok(list!(vec![
                MalVal::Symbol("splice-unquote".to_string()),
                read_form(rdr)?,
            ]))
        }
        "^" => {
            rdr.skip();
            let meta = read_form(rdr)?;
            Ok(list!(vec![
                MalVal::Symbol("with-meta".to_string()),
                read_form(rdr)?,
                meta,
            ]))
        }
        "@" => {
            rdr.skip();
            Ok(list!(vec![
                MalVal::Symbol("deref".to_string()),
                read_form(rdr)?,
            ]))
        }
        "(" => read_list(rdr),
        "[" => read_vector(rdr),
        "{" => read_hmap(rdr),
        "]" => Err(MalError::Error("unexpected ']'".to_string())),
        ")" => Err(MalError::Error("unexpected ')'".to_string())),
        "}" => Err(MalError::Error("unexpected '}'".to_string())),
        _ => read_atom(rdr),
    }
}

fn read_seq(rdr: &mut Reader, c: &str) -> MalReturn {
    let mut seq = vec![];
    let mut hmap = HashMap::new();

    // Skip opening symbol: {/[/(
    rdr.skip();

    loop {
        let token = match rdr.peek() {
            Ok(t) => t,
            Err(_) => return Err(MalError::Error(format!("expected '{}', got EOF", c))),
        };

        if token.eq(c) {
            // Skip closing symbol: }/]/)
            rdr.skip();
            break;
        }

        match c {
            "}" => {
                let k = match read_form(rdr) {
                    Ok(MalVal::Str(key)) => key,
                    Ok(v) => v.pr_str(true),
                    Err(e) => return Err(e),
                };
                let v = read_form(rdr)?;

                hmap.insert(k, v);
            }
            _ => {
                seq.push(read_form(rdr)?);
            }
        }
    }

    match c {
        ")" => Ok(list!(seq)),
        "]" => Ok(MalVal::Vector(Rc::new(seq))),
        "}" => Ok(MalVal::Hashmap(Rc::new(hmap))),
        _ => Err(MalError::Error(format!("unexpected end of input '{}'", c))),
    }
}

fn read_list(rdr: &mut Reader) -> MalReturn {
    read_seq(rdr, ")")
}

fn read_vector(rdr: &mut Reader) -> MalReturn {
    read_seq(rdr, "]")
}

fn read_hmap(rdr: &mut Reader) -> MalReturn {
    read_seq(rdr, "}")
}

fn read_atom(rdr: &mut Reader) -> MalReturn {
    let re_int = Regex::new(r#"^-?[0-9]+$"#).unwrap();
    let re_str = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();

    let token = rdr.next()?;
    match token {
        "nil" => Ok(MalVal::Nil),
        "true" => Ok(MalVal::Bool(true)),
        "false" => Ok(MalVal::Bool(false)),
        _ => {
            if re_int.is_match(token) {
                Ok(MalVal::Int(token.parse().unwrap()))
            } else if re_str.is_match(token) {
                Ok(MalVal::Str(unescape_str(&token[1..token.len() - 1])))
            } else if token.starts_with("\"") {
                Err(MalError::Error("expected \" at end of input".to_string()))
            } else if token.starts_with(":") {
                Ok(MalVal::Str(format!(
                    "\u{29e}{}",
                    String::from(token.strip_prefix(':').unwrap())
                )))
            } else {
                Ok(MalVal::Symbol(token.to_string()))
            }
        }
    }
}
