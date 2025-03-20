use crate::val::*;

fn sqrt(vm: &mut VM, a: Val) -> Result<Val, String> {
    match &a {
        Val::Float(a) => Val::Float(a.sqrt()),
        Val::Int(a) => Val::Int(a.sqrt()),
        _ => {
            return Err("Expected number");
        }
    }
}

fn register(vm: VM) {
    vm.vars.insert("sqrt", Val::func(sqrt));
}
