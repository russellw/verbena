use crate::val::*;
use crate::vm::*;

fn sqrt(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match &a {
        Val::Float(a) => Val::Float(a.sqrt()),
        Val::Int(a) => Val::Int(a.sqrt()),
        _ => {
            return Err("Expected number".to_string());
        }
    };
    Ok(r)
}

fn add1(vm: &mut VM, name: &str, f: fn(&mut VM, Val) -> Result<Val, String>) {
    vm.vars.insert(name.to_string(), Val::func(f));
}

fn register(vm: &mut VM) {
    add1(vm, "sqrt", sqrt);
}
