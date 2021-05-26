mod argparser;

use simple_logger::SimpleLogger;
use std::env;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Trace).init().unwrap();

    // XXX have to register atexit func?
    /* */

    let myname = std::env::current_exe();
    log::trace!("{:?}", myname);

    // XXX must set sighandlers 
    /* */
    arg_parse();
}

pub fn arg_parse() -> u64 {
    use argparser::ArgType;

    for (ix,arg) in env::args().enumerate() {
        log::trace!("argv[{}] is {}", ix, arg);
        if ix == 0 {
            log::trace!("skip this arg");
            continue;
        }
        match argparser::arg_type(&arg) {
          ArgType::ArgBsd => {
            log::trace!("{}: type ArgBsd", arg);
          },
          ArgType::ArgSysv => {
            log::trace!("{}: type ArgSysv", arg);
          },
          ArgType::ArgGnu => {
            log::trace!("{}: type ArgGnu", arg);
          },
          ArgType::ArgPgrp | ArgType::ArgSess | ArgType::ArgPid => {
            log::trace!("{}: type Arg{{Pgrp|Sess|Pid}}", arg);
          },
          ArgType::ArgFail => {
            log::trace!("{}: type ArgFail", arg);
            do_help(String::from(""), -1,);
          },
        }
    }
    0
}

pub fn do_help(opt: String, rc: i32) {
  log::trace!("do_help called with opt:{}, rc:{}", opt, rc);
  std::process::exit(rc);
}
