use std::process::exit;

use super::ast::nodes::Type;

pub struct Linker {
    funcs: Vec<LinkerFunc>,
}
impl Linker {
    pub fn new() -> Linker {
        Linker { funcs: Vec::new() }
    }

    pub fn add_func(&mut self, function: &LinkerFunc) {
        // Check for conflicting signature
        for func in &self.funcs {
            let ret_type = &func.ret_type;

            if function.get_signature() == func.get_signature() {
                eprintln!("Error: Duplicate function {}", function.name);
                exit(1)
            }

            if function.ret_type != *ret_type && function.name == func.name && function.arg_types == func.arg_types {
                eprintln!("Error: Cannot overload function {} based on return types", function.name);
                exit(1)
            }
        }

        self.funcs.push(function.clone())
    }

    pub fn get_func(&self, name: &String, arg_types: &Vec<Type>) -> Option<LinkerFunc> {
        for func in &self.funcs {
            if func.name == *name && func.arg_types == *arg_types {
                return Some(func.clone());
            }
        }
        None
    }

    pub fn get_funcs(&self) -> &Vec<LinkerFunc> {
        &self.funcs
    }
}

#[derive(Debug, Clone)]
pub struct LinkerFunc {
    ret_type: Type,
    name: String,
    arg_types: Vec<Type>,
    pub code: String,
}
impl LinkerFunc {
    pub fn new(ret_type: &Type, name: &String, arg_types: &Vec<Type>, code: &String) -> Self {
        Self {
            ret_type: ret_type.clone(),
            name: name.clone(),
            arg_types: arg_types.clone(),
            code: code.clone(),
        }
    }

    pub fn get_signature(&self) -> String {
        let mut s = String::new();

        s += format!("_Hx{}{}", self.name.len(), self.name).as_str();

        let (ret_len, ret) = LinkerFunc::encode_type(&self.ret_type);
        if ret_len == usize::MAX {
            s += ret.as_str();
        } else {
            s += format!("_{}{}", ret_len, ret).as_str()
        }

        for arg in &self.arg_types {
            let (var_len, var) = LinkerFunc::encode_type(arg);
            if var_len == usize::MAX {
                s += var.as_str();
            } else {
                s += format!("_{}{}", var_len, var).as_str()
            }
        }

        s
    }

    fn encode_type(typ: &Type) -> (usize, String) {
        let s;
        let len;
        let mut is_ident = false;

        match typ {
            Type::Named(name) => {
                match name.as_str() {
                    "void" => s = String::from("v"),
                    "int8" => s = String::from("i8"),
                    "int16" => s = String::from("i16"),
                    "int32" => s = String::from("i32"),
                    "int64" => s = String::from("i64"),

                    "uint8" => s = String::from("u8"),
                    "uint16" => s = String::from("u16"),
                    "uint32" => s = String::from("u32"),
                    "uint64" => s = String::from("u64"),

                    "float32" => s = String::from("f32"),
                    "float64" => s = String::from("f64"),

                    "string" => s = String::from("s"),
                    "char" => s = String::from("c"),

                    _ => {
                        s = name.clone();
                        is_ident = true
                    }
                }

                len = name.len()
            }
            Type::Ptr(_) => todo!(),
            Type::Arr(_) => todo!(),
            Type::Const(_) => todo!(),
        }

        (
            {
                if is_ident {
                    len
                } else {
                    usize::MAX
                }
            },
            s,
        )
    }
}
