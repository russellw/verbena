use crate::val::*;
use crate::vm::*;

fn sqrt(vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match &a {
        Val::Float(a) => Val::Float(a.sqrt()),
        Val::Int(a) => Val::Int(a.sqrt()),
        _ => {
            return Err("Expected number".to_string());
        }
    };
    Ok(r)
}

fn register(vm: VM) {
    vm.vars.insert("sqrt", Val::func(sqrt));
}
