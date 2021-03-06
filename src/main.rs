extern crate good_boi;

use std::io::Write;
use std::error::Error;
use std::collections::HashSet;
use good_boi::{Cartridge, Core, Bus};

fn main() {
    // let rom = include_bytes!("../gb-test-roms/cpu_instrs/cpu_instrs.gb");
    // let rom = include_bytes!("../gb-test-roms/cpu_instrs/individual/02-interrupts.gb");
    let rom = include_bytes!("../gb-test-roms/cpu_instrs/individual/04-op r,imm.gb");
    // let rom = include_bytes!("../gb-test-roms/cpu_instrs/individual/06-ld r,r.gb");
    // let rom = include_bytes!("/tmp/test.gb");

    let cartridge = Cartridge::load(&rom[..]).unwrap();
    let bus = Bus::new(cartridge);

    Debugger::new(bus).run();
}

struct Debugger {
    core: Core,
    breakpoints: HashSet<u16>,
}

impl Debugger {
    fn new(bus: Bus) -> Self {
        Self {
            core: Core::new(bus),
            breakpoints: HashSet::new(),
        }
    }

    fn run(&mut self) {
        self.core.print_state();

        // loop {
        //     core.step();
        //     // core.print_state();
        // }

        loop {
            let mut input = String::new();

            print!("> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.split_whitespace().collect::<Vec<&str>>();

            let result = match &*input {
                ["b", addr] => self.add_breakpoint(addr),
                ["p", addr] => self.print_mem_u8(addr),
                ["pp", addr] => self.print_mem_u16(addr),
                ["r"] => self.run_forever(),
                ["r", addr] => self.run_until(addr),
                ["rp"] => self.run_past(&self.core.pc().to_string()),
                ["rp", addr] => self.run_past(addr),
                ["w", addr, value] => self.write_mem_u8(addr, value),
                ["ww", addr, value] => self.write_mem_u16(addr, value),
                [] | ["n"] => self.single_step(),
                _ => Err("Unknown command".into()),
            };

            if let Err(err) = result {
                println!("❌ {}", err);
            }
        }
    }

    fn run_past(&mut self, addr: &str) -> Result<(), Box<Error>> {
        let addr = u16::from_str_radix(addr, 16)?;

        while self.core.pc() <= addr {
            self.core.step();
        }

        self.core.print_state();

        Ok(())
    }

    fn single_step(&mut self) -> Result<(), Box<Error>> {
        self.core.step();
        self.core.print_state();
        Ok(())
    }

    fn run_forever(&mut self) -> Result<(), Box<Error>> {
        loop {
            self.core.step();
            // self.core.print_state();

            if self.breakpoints.contains(&self.core.pc()) {
                println!("Stopping at breakpoint.");
                break Ok(());
            }
        }
    }

    fn add_breakpoint(&mut self, addr: &str) -> Result<(), Box<Error>> {
        let addr = u16::from_str_radix(addr, 16)?;

        self.breakpoints.insert(addr);

        Ok(())
    }

    fn run_until(&mut self, addr: &str) -> Result<(), Box<Error>> {
        let addr = u16::from_str_radix(addr, 16)?;

        loop {
            self.core.step();

            if addr == self.core.pc() {
                break;
            }
        }

        self.core.print_state();
        Ok(())
    }

    fn print_mem_u8(&mut self, addr: &str) -> Result<(), Box<Error>> {
        let addr = u16::from_str_radix(addr, 16)?;
        let value = self.core.peek_mem_u8(addr);

        println!("[{:04X}] = {:02X}", addr, value);

        Ok(())
    }

    fn print_mem_u16(&mut self, addr: &str) -> Result<(), Box<Error>> {
        let addr = u16::from_str_radix(addr, 16)?;
        let value = self.core.peek_mem_u16(addr);

        println!("[{:04X}] = {:04X}", addr, value);

        Ok(())
    }

    fn write_mem_u8(&mut self, addr: &str, value: &str) -> Result<(), Box<Error>> {
        let addr = u16::from_str_radix(addr, 16)?;
        let value = u8::from_str_radix(value, 16)?;

        self.core.write_mem_u8(addr, value);

        dbg!(addr);

        println!("[{:04X}] = {:02X}", addr, value);

        Ok(())
    }

    fn write_mem_u16(&mut self, addr: &str, value: &str) -> Result<(), Box<Error>> {
        let addr = u16::from_str_radix(addr, 16)?;
        let value = u16::from_str_radix(value, 16)?;

        self.core.write_mem_u16(addr, value);

        dbg!(addr);

        println!("[{:04X}] = {:02X}", addr, value);

        Ok(())
    }
}
