pub struct Context {
    pub position: u32,
    pub line: u32,
    pub code_source: String
}

impl Context {
    pub fn new () -> Self {
        Context { code_source: "void".to_string(), line: 0, position: 0 }
    }

    pub fn set_code_source (&mut self, code_source: String) {
        self.code_source = code_source;
    }
}
