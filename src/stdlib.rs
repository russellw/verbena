use crate::val::*;

fn sqrt(a: Val) -> Result<Val, String> {
    match &a {
        Val::Float(a) => Val::Float(a.sqrt()),
        Val::Int(a) => Val::Int(a.sqrt()),
        _ => {
            return Err(self.err("Expected number"));
        }
    }
}

fn register(vm: VM) {
    vm.vars.insert("sqrt", Val::func(sqrt));
}
