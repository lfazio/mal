use crate::environment::MalEnv;
use crate::error::MalError;
use crate::printer::pr_str;
use crate::types::MalVal;

pub fn eval(input: Result<MalVal, MalError>, env: &mut MalEnv) -> Result<MalVal, MalError> {
    let ast = input?;

    if let Ok(MalVal::Bool(debug)) = env.get(&"DEBUG-EVAL".to_string()) {
        if debug {
            print!("EVAL: ");
            pr_str(&ast);
        }
    }

    match ast.clone() {
        MalVal::Symbol(k) => env.get(&k),
        MalVal::Vector(mut v) => {
            for v in v.iter_mut() {
                *v = eval(Ok(v.clone()), env)?;
            }

            Ok(MalVal::Vector(v))
        }
        MalVal::List(mut l) => {
            if l.is_empty() {
                return Ok(ast);
            }

            for e in l.iter_mut() {
                match eval(Ok(e.clone()), env) {
                    Ok(v) => *e = v,
                    Err(e) => return Err(e),
                }
            }

            let func = l.remove(0);
            match func.apply(l) {
                Ok(v) => Ok(v),
                Err(e) => Err(MalError::Error(e)),
            }
        }
        MalVal::Hashmap(mut h) => {
            let entries: Vec<_> = h.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            for (k, v) in entries {
                h.entry(k)
                    .and_modify(|val| *val = eval(Ok(v), env).unwrap());
            }

            Ok(MalVal::Hashmap(h))
        }
        _ => Ok(ast),
    }
}
