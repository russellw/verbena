use crate::val::*;
use crate::vm::*;

const ARG_NUM: &str = "Expected number";

fn sqrt(vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match &a {
        Val::Float(a) => Val::Float(a.sqrt()),
        Val::Int(a) => Val::Int(a.sqrt()),
        _ => {
            return Err(ARG_NUM.to_string());
        }
    };
    Ok(r)
}

fn register(vm: VM) {
    vm.vars.insert("sqrt", Val::func(sqrt));
}
