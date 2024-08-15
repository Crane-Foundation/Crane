// Define the enum for basic types
#[derive(Debug, Clone)]
enum CraneType {
    //primitive types
    Char,
    U16,
    Short { signed: bool },
    Long { signed: bool },
    Void,
    Bool,
    //derived types
    Pointer { pointee: Box<CraneType> },
    Array { element: Box<CraneType>, size: u32 },
}

// Define the struct for a variable
struct Variable {
    name: String,
    var_type: CraneType,
    size: u32,
    offset: u32,
    value: u64,
}

impl Variable {
    fn new(name: String, var_type: CraneType, size: u32, offset: u32, value: u64) -> Variable {
        Variable {
            name,
            var_type,
            size,
            offset,
            value,
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn var_type(&self) -> CraneType {
        self.var_type.clone()
    }
    fn size(&self) -> u32 {
        self.size
    }
    fn offset(&self) -> u32 {
        self.offset
    }
    fn value(&self) -> u64 {
        self.value
    }
    fn set_value(&mut self, value: u64) {
        self.value = value;
    }
}
