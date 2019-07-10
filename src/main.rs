use std::fs;

extern crate clap;
use clap::{Arg, App};

// Decode n bytes at position
macro_rules! decode {
    {1, $mem:expr, $pos:expr} =>
    {{
         $pos = $pos + 1;
         let ret : u8 = $mem[$pos];
         ret
     }};
    {2, $mem:expr, $pos:expr} =>
    {{
         $pos = $pos + 1;
         let lower : u16 = $mem[$pos] as u16;
         $pos = $pos + 1;
         let upper : u16 = $mem[$pos] as u16;
         let ret : u16 = (upper << 8) | lower;
         ret
     }};
}

fn main() {
    let matches = App::new("Simple SNES Emulator")
                          .version("0.1")
                          .author("Vinz <vincent.siles@ens-lyon.org>")
                          .about("Will do awesome things")
                          // .arg(Arg::with_name("config")
                          //      .short("c")
                          //      .long("config")
                          //      .value_name("FILE")
                          //      .help("Sets a custom config file")
                          //      .takes_value(true))
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();

    // // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let config = matches.value_of("config").unwrap_or("default.conf");
    // println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let filename = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", filename);

    // // Vary the output based on how many times the user used the "verbose" flag
    // // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    // match matches.occurrences_of("v") {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }

    
    let rom = fs::read(filename).expect("Can't read input file");
    let rom_len = rom.len();

    let mut i = 0;

    while i < rom_len {
        let b = rom[i];
        match b {
            0x18 => println!("CLC"),
            0x78 => println!("SEI"),
            0xfb => println!("XCE"),

            0xcb => println!("WAI"),
            0x40 => println!("RTI"),

            0xe8 => println!("INX"),
            0xc8 => println!("INY"),
            
            0xea => println!("NOP"),
            0x42 => {
                let _ = decode!(1, rom, i);
                println!("WDM/NOP")
            },

            0x4c => {
                let addr = decode!(2, rom, i);
                println!("JMP {:#04x}", addr)
            },

            0x8d => {
                let addr = decode!(2, rom, i);
                println!("STA {:#04x}", addr)
            },
            0x9c => {
                let addr = decode!(2, rom, i);
                println!("STZ {:#04x}", addr)
            },

            0xa2 => {
                let val = decode!(1, rom, i);
                println!("LDX {:#02x}", val)
            },
            0xa9 => {
                let val = decode!(1, rom, i);
                println!("LDA #{:02x}", val)
            },
            0xad => {
                let addr = decode!(2, rom, i);
                println!("LDA #{:04x}", addr)
            },
            0xbd => {
                let addr = decode!(2, rom, i);
                println!("LDA #{:04x},X", addr)
            },

            0x90 => {
                let val = decode!(1, rom, i);
                println!("BCC #{:02x}", val)
            },

            0xe0 => {
                let val = decode!(1, rom, i);
                println!("CPX #{:02x}", val)
            },
            _ => break,
        }
        i = i + 1;
    }

    let mut br = i % 8;
    while i < rom_len {
        let b = rom[i];
        i = i + 1;
        print!("{:02x} ", b);
        br = br + 1;
        if br >= 8 {
            println!("");
            br = 0
        }
    }
}
