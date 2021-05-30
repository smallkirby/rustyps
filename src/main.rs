#[macro_use]
extern crate scan_fmt;

pub mod argparser;
pub mod display;
pub mod helper;
pub mod readproc;

use simple_logger::SimpleLogger;

#[derive(Debug)]
pub struct Ps {
  parser: argparser::PsParser,
}

impl Ps {
  fn new() -> Ps {
    Ps {
      parser: argparser::PsParser::from(std::env::args()),
    }
  }
  pub fn run(&mut self) -> i32 {
    match self.parser.parse() {
      Ok(()) => log::trace!("success parse"),
      Err(msg) => println!("Err: {}", msg),
    }
    self.arg_check_conflicts();

    log::trace!("===== ps output follows ====");
    self.init_output();
    self.lists_and_needs();

    match display::simple_spew(&self.parser) {
      Ok(()) => log::trace!("simple_spew finish"),
      Err(msg) => println!("{}", msg),
    }
    return 0;
  }

  // XXX
  pub fn arg_check_conflicts(&mut self) {
    return;
  }
  // XXX
  pub fn init_output(&mut self) {}

  // XXX
  pub fn lists_and_needs(&mut self) {}
}

fn main() {
  SimpleLogger::new()
    .with_level(log::LevelFilter::Warn)
    .init()
    .unwrap();

  // XXX have to register atexit func?
  /* */

  let myname = std::env::current_exe();
  //log::trace!("{:?}", myname);

  // XXX must set sighandlers
  /* */
  let mut ps = Ps::new();
  ps.run();
}

pub fn do_help(opt: String, rc: i32) {
  log::trace!("do_help called with opt:{}, rc:{}", opt, rc);
  std::process::exit(rc);
}
