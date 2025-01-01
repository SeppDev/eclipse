enum ErrorCode {
    VariableNotFound = 1
}

pub fn variable_not_found(name: &String) -> String {
    format!("Cannot find variable named {name}")
}

pub fn variable_already_declared(name: &String) -> String {
    format!("Variable is already declared {name}")
}
