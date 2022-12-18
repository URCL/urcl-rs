mod console;
use console::Console;
use super::super::*;

pub trait Device {
    fn connect(&mut self, host: &mut DeviceHost) -> ();
}

pub struct DeviceHost {
    console: console::Console,
    // out_ports: Vec<fn(host: Device, u64)>,
    // in_ports: Vec<Box<dyn Fn() -> u64>>,
}//rip
// we could take a break from ports and add other bits than 64
use std::fmt::{Formatter, Result, Debug};
impl Debug for DeviceHost {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        "DeviceHost lol (clearly this is the best formatting)".fmt(fmt)
    }
}

impl DeviceHost {
    // pub fn add_out<F: Fn(u64)>(&mut self, port_number: u8, f: F) {
    //     self.out_ports[port_number as usize] = Box::new(f);
    // }
    // pub fn add_in<'a, F: 'a + Fn() -> u64>(&'a mut self, port_number: u8, f: F) {
    //     self.in_ports[port_number as usize] = Box::new(f);
    // }
        
    pub fn out(&mut self, port: u64, value: u64) {
        match port {
            1 => self.console.outtext(value),
            2 => self.console.outnumb(value),
            24 => self.console.outint(value),
            27 => self.console.outhex(value),
            _ => {todo!("unsupported port {}", port)}
        }
    }
    // its good enough
    pub fn show(&self) {
        jsprintln!("{}", self.console.get_output());
    }

    pub fn new() -> Self {
        // let mut out_ports: Vec<Box<dyn Fn(u64)>> = Vec::new();
        // out_ports.resize_with(256, || Box::new(|_: u64| {}));
        // // out_ports.
        // let mut in_ports: Vec<Box<dyn Fn() -> u64>> = Vec::new();
        // in_ports.resize_with(256, || Box::new(|| {0}));

        // Self { out_ports, in_ports }
        Self { console: Console::new()  }
    }
}
