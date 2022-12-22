pub struct Console { // console::console::console::console::console::console::console::console::console
    output: String
}
impl Console {
    pub fn new() -> Self {
        Self { output: String::new() }
    }

    pub fn outtext(&mut self, value: u64){
        if let Some(c) = char::from_u32(value as u32) {
            self.output.push(c);
        } else {
            // todo: report error
        }
    }
    pub fn outnumb(&mut self, value: u64){
        self.output.push_str(&value.to_string());
    }
    pub fn outhex(&mut self, value: u64){
        self.output.push_str(&format!("{:X}", value));
    }
    pub fn outint(&mut self, value: u64){
        self.output.push_str(&(value as i64).to_string())
    }

    pub fn get_output(&self) -> &str {
        &self.output
    }

    pub fn clear_output(&mut self, keep: usize) {
        let mut new_output = String::new();
        if keep > 0 {
            let mut chars = self.output.chars();
            while chars.as_str().len() > keep {
                chars.next();
            }
            new_output = chars.as_str().to_owned();
        }

        self.output = new_output;
    }
}
// epic rust
