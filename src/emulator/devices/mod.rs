mod console;
mod screen;
use console::Console;
use self::screen::Screen;
use super::super::*;

use strum_macros::EnumString;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, EnumString, FromPrimitive)]
#[repr(u8)]
#[allow(dead_code, non_camel_case_types)]
pub enum IOPort {
    // General
    CPUBUS, TEXT, NUMB, SUPPORTED = 5, SPECIAL, PROFILE,
    // Graphics
    X, Y, COLOR, BUFFER, G_SPECIAL = 15,
    // Text
    ASCII, CHAR5, CHAR6, ASCII7, UTF8, UTF16, UTF32, T_SPECIAL = 23,
    // Numbers
    INT, UINT, BIN, HEX, FLOAT, FIXED, N_SPECIAL=31,
    // Storage
    ADDR, BUS, PAGE, S_SPECIAL=39,
    // Miscellaneous
    RNG, NOTE, INSTR, NLEG, WAIT, NADDR, DATA, M_SPECIAL,
    // User defined
    UD1, UD2, UD3, UD4, UD5, UD6, UD7, UD8, UD9, UD10, UD11, UD12, UD13, UD14, UD15, UD16,

    GAMEPAD, AXIS, GAMEPAD_INFO,
    KEY,
    MOUSE_X, MOUSE_Y, MOUSE_DX, MOUSE_DY,
    MOUSE_DWHEEL,
    MOUSE_BUTTONS,
    FILE,
}


pub trait Device {
    fn connect(&mut self, host: &mut DeviceHost) -> ();
}

pub struct DeviceHost {
    console: console::Console,
    screen: screen::Screen,
}//rip
// we could take a break from ports and add other bits than 64
use std::fmt::{Formatter, Result, Debug};
impl Debug for DeviceHost {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        "DeviceHost lol (clearly this is the best formatting)".fmt(fmt)
    }
}

impl DeviceHost {        
    pub fn out(&mut self, _port: u64, value: u64) {
        let Some(port) = FromPrimitive::from_u64(value) else {return;};
        match port {
            IOPort::TEXT => self.console.outtext(value),
            IOPort::NUMB => self.console.outnumb(value),
            IOPort::INT => self.console.outint(value),
            IOPort::HEX => self.console.outhex(value),
            IOPort::X => self.screen.out_x(value),
            IOPort::Y => self.screen.out_y(value),
            IOPort::COLOR => self.screen.out_color(value),
            _ => {todo!("unsupported port {:?}", port)}
        }
    }

    pub fn show(&mut self) {
        self.console.clear_output(10_000);
        jsprintln!("{}", self.console.get_output());
        out_screen(self.screen.width(), self.screen.height(), self.screen.pixels());
    }

    pub fn new() -> Self {
        Self { console: Console::new(), screen: Screen::new(32, 32) }
    }
}
