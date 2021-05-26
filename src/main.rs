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
    for (ix,arg) in env::args().enumerate() {
        log::trace!("argv[{}] is {}", ix, arg);
        if ix == 0 {
            log::trace!("skip this arg");
            continue;
        }
        match arg_type(&arg) {
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
          },
          _ => {
            log::error!("some thing went wrong when parsing arg: {}", arg);
            panic!();
          },
        }
    }
    0
}

pub fn arg_type(arg: &String) -> ArgType {
    let c0 = arg.chars().nth(0);
    match c0 {
      None => ArgType::ArgFail,
      Some(c0) => match c0 {
        'a'..='z' => ArgType::ArgBsd,
        'A'..='Z' => ArgType::ArgBsd,
        '0'..='9' => ArgType::ArgPid,
        '+' => ArgType::ArgSess,
        '-' => {
          let c1 = arg.chars().nth(1);
          match c1 {
            None => ArgType::ArgFail,
            Some(c1) => match c1 {
              'a'..='z' => ArgType::ArgSysv,
              'A'..='Z' => ArgType::ArgSysv,
              '0'..='9' => ArgType::ArgPgrp,
              '-' => {
                let c2 = arg.chars().nth(2);
                match c2 {
                  None => ArgType::ArgFail,
                  Some(c2) => match c2 {
                    'a'..='z' => ArgType::ArgGnu,
                    'A'..='Z' => ArgType::ArgGnu,
                    _ => ArgType::ArgFail,
                  }
                }
              },
              _ => ArgType::ArgFail,
            }
          }
        },
        _ => ArgType::ArgFail,
      }
    }
}

#[derive(Debug, PartialEq)]
pub enum ArgType {
    ArgGnu,
    ArgEnd, // NOT USED
    ArgPgrp,
    ArgSysv,
    ArgPid,
    ArgBsd,
    ArgFail,
    ArgSess,
}

#[cfg(test)]
mod tests {
  #[test]
  fn arg_type_parser() {
    assert_eq!(super::arg_type(&String::from("")), super::ArgType::ArgFail);
    assert_eq!(super::arg_type(&String::from("?")), super::ArgType::ArgFail);
    assert_eq!(super::arg_type(&String::from("ax")), super::ArgType::ArgBsd);
    assert_eq!(super::arg_type(&String::from("Ax")), super::ArgType::ArgBsd);
    assert_eq!(super::arg_type(&String::from("3")), super::ArgType::ArgPid);
    assert_eq!(super::arg_type(&String::from("+3")), super::ArgType::ArgSess);
    assert_eq!(super::arg_type(&String::from("-3")), super::ArgType::ArgPgrp);
    assert_eq!(super::arg_type(&String::from("-ax")), super::ArgType::ArgSysv);
    assert_eq!(super::arg_type(&String::from("-")), super::ArgType::ArgFail);
    assert_eq!(super::arg_type(&String::from("- ")), super::ArgType::ArgFail);
    assert_eq!(super::arg_type(&String::from("--ax")), super::ArgType::ArgGnu);
    assert_eq!(super::arg_type(&String::from("--AX")), super::ArgType::ArgGnu);
    assert_eq!(super::arg_type(&String::from("---")), super::ArgType::ArgFail);
    assert_eq!(super::arg_type(&String::from("--")), super::ArgType::ArgFail);
    assert_eq!(super::arg_type(&String::from("--3")), super::ArgType::ArgFail);
  }
}