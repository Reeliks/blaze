pub struct Context {
    pub code_source: String,
    pub position: u64,
    pub line: u64
}

impl Context {
    fn new() -> Self {
        Context {
            code_source: "void".to_string(),
            position: 0,
            line: 0
        }
    }

    pub fn set_code_source(&mut self, code_source: String) {
        self.code_source = code_source;
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
