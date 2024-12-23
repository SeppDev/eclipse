use crate::compiler::{
    analyzer::{IRType, IRValue},
    parser::{ArithmeticOperator, CompareOperator},
    string::BetterString,
    types::Type, POINTER_WITH,
};

// pub fn codegen(program: IRProgram) -> String {
//     let mut source = BetterString::new();

//     for (key, value) in program.static_strings {
//         source.pushln(format!(
//             "@{key} = private constant [ {} x i8 ] c\"{value}\\00\"",
//             value.len() + 1
//         ));
//     }

//     return source.to_string();
// }

#[derive(Debug)]
pub struct CodeGen {
    body: BetterString,
    functions: Vec<FunctionOperations>,
}
impl CodeGen {
    pub fn new() -> Self {
        Self {
            body: BetterString::new(),
            functions: Vec::new()
        }
    }
    pub fn generate(mut self) -> String {
        self.body
            .pushln("target triple = \"x86_64-pc-windows-unkown\"\n");

        self.body.pushln(
            "declare i32 @printf(i8*, ...)

declare i32 @rand()
declare i32 @sleep(i32)
declare i32 @usleep(i32)
declare i32 @fflush(ptr)
@stdout = external global ptr
@format = private constant [4 x i8] c\"%d\\0A\\00\"

define void @print(i32 %x) {
start:
    %fmt_ptr = getelementptr [5 x i8], ptr @format, i32 0, i32 0
    call i32 @printf(i8* %fmt_ptr, i32 %x)

    %stdout_ptr = load ptr, ptr @stdout
    call i32 @fflush(ptr %stdout_ptr)

    ret void
}",
        );

        for function in self.functions {
            self.body.push(function.to_string());
            self.body.pushln("}");
        }

        self.body.to_string()
    }
    pub fn insert(&mut self, function: FunctionOperations) {
        self.functions.push(function);
    }
    pub fn new_function(
        &self,
        key: &String,
        return_type: &Type,
        parameters: &Vec<(String, IRType)>,
    ) -> FunctionOperations {
        let mut body = BetterString::new();
        let mut params: Vec<String> = Vec::new();

        let return_type = if return_type.base.is_basic() {
            return_type.convert()
        } else {
            let return_type = return_type.convert();
            params.push(format!("ptr sret({return_type}) %0"));
            IRType::Void
        };

        params.extend(
            parameters
                .into_iter()
                .map(|(key, data_type)| format!("{} %{key}", data_type))
                .collect::<Vec<String>>(),
        );

        body.pushln(format!(
            "define {return_type} @{key}({}) {{",
            params.join(", ")
        ));
        body.pushln("start:");

        FunctionOperations {
            body,
        }
    }
}

#[derive(Debug)]
pub struct FunctionOperations {
    body: BetterString,
}
impl FunctionOperations {
    pub fn to_string(self) -> String {
        self.body.to_string()
    }
    pub fn allocate(&mut self, destination: &String, data_type: &IRType) {
        self.body
            .pushln(format!("\t%{destination} = alloca {data_type}"));
    }
    pub fn store(&mut self, data_type: &IRType, value: &IRValue, destination: &String) {
        self.body
            .pushln(format!("\tstore {data_type} {value}, ptr %{destination}"));
    }
    pub fn store_from_pointer(&mut self, data_type: &IRType, pointer: &String, destination: &String) {
        self.body
            .pushln(format!("\tstore {data_type} %{pointer}, ptr %{destination}"));
    }
    pub fn xor_boolean(&mut self, destination: &String, value: &IRValue) {
        self.body
            .pushln(format!("\t%{destination} = xor i1 {value}, true"));
    }
    pub fn call(&mut self, function: &String, return_type: &IRType, arguments: IRValue) {
        self.body
            .pushln(format!("\tcall {return_type} @{function}({arguments})"));
    }
    pub fn store_call(
        &mut self,
        destination: &String,
        function: &String,
        return_type: &IRType,
        arguments: IRValue,
    ) {
        self.body.pushln(format!(
            "\t%{destination} = call {return_type} @{function}({arguments})"
        ));
    }
    pub fn label(&mut self, label: &String) {
        self.body.pushln(format!("{}:", label));
    }
    // pub fn getelementptr(&mut self, destination: &String, operation: ElemmentPointerOperation) {
    //     self.body
    //         .pushln(format!("%{destination} = getelementptr {operation}"));
    // }
    pub fn getelementptr_inbounds(
        &mut self,
        destination: &String,
        data_type: &IRType,
        from: &String,
        index_type: &IRType,
        index: &IRValue,
    ) {
        self.body
            .pushln(format!("\t%{destination} = getelementptr inbounds {data_type}, ptr %{from}, i32 0, {index_type} {index}"));
    }
    pub fn memcpy(&mut self, destination: &String, from: &String, size: &usize, volatile: bool) {
        self.body.pushln(format!("\tcall void @llvm.memcpy.p0.p0.i{POINTER_WITH}(ptr %{destination}, ptr %{from}), i{POINTER_WITH} {size}, i1 {volatile}"))
    }
    pub fn load(&mut self, destination: &String, destination_type: &IRType, value: &IRValue) {
        self.body.pushln(format!(
            "\t%{destination} = load {destination_type}, ptr {value}"
        ));
    }
    pub fn load_from_pointer(&mut self, destination: &String, destination_type: &IRType, pointer: &String) {
        self.body.pushln(format!(
            "\t%{destination} = load {destination_type}, ptr %{pointer}"
        ));
    }
    pub fn binary_operation(
        &mut self,
        destination: &String,
        operator: &ArithmeticOperator,
        data_type: &IRType,
        first: &IRValue,
        second: &IRValue,
    ) {
        self.body.pushln(format!(
            "\t%{destination} = {} {data_type} {first}, {second}",
            operator.convert(&data_type)
        ));
    }
    pub fn compare_operation(
        &mut self,
        destination: &String,
        operator: &CompareOperator,
        data_type: &IRType,
        first: &IRValue,
        second: &IRValue,
    ) {
        self.body.pushln(format!(
            "\t%{destination} = {} {data_type} {first}, {second}",
            operator.convert(&data_type)
        ));
    }
    pub fn r#return(&mut self, data_type: &IRType, value: &IRValue) {
        self.body.pushln(format!("\tret {} {}", data_type, value));
    }
    pub fn void_return(&mut self) {
        self.body.pushln(format!("\tret void"));
    }
    pub fn branch(&mut self, condition: &IRValue, yes: &String, no: &String) {
        self.body
            .pushln(format!("\tbr i1 {condition}, label %{yes}, label %{no}"));
    }
    pub fn goto(&mut self, label: &String) {
        self.body.pushln(format!("\tbr label %{label}"));
    }
}
