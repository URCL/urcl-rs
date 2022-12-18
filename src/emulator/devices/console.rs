use super::Device;
use super::DeviceHost;

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
}
// epic rust

impl Device for Console {
    fn connect(&mut self, host: &mut DeviceHost) -> () {
        // host.add_out(1, |value| self.outtext(value));
        // host.add_out(2, |value| self.outnumb(value));
        // host.add_out(27, |value| self.outint(value));
        // host.add_out(27, |value| self.outnumb(value));
    }
}
