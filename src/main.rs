#[cfg(all(target_os = "linux",
          any(target_arch = "x86",
              target_arch = "x86_64")),
)]

extern crate clap;
extern crate libc;
extern crate nix;
extern crate serde;

#[macro_use] 
extern crate serde_derive;

mod ptrace;
use ptrace::helpers;

fn main() {
    let matches = App::new("jd")
                        .version("0.1.0")
                        .author("Alan <ex0dus@codemuch.tech")
                        .about("process trace to json utility")
                        .arg(Arg::with_name("emit")
                             .short("e")
                             .long("emit")
                             .value_name("OUTPUT_TYPE")
                             .required(false)
                             .help("sets type of output to emit"))
                        .arg(Arg::with_name("command")
                             .short("c")
                             .long("command")
                             .value_name("COMMAND")
                             .min_value(1)
                             .required(true)
                             .help("sets command to trace"))
                        .get_matches();

    // retrieve commands
    let commands: Vec<_> = matches.values_of("command")
                                  .unwrap().collect();

    // parse emit type
    match matches.value_of("emit") {
        "raw"   => {},
        "json"  => {},
        _       => {
            panic!("unknown emit output type");
        }
    }
}
