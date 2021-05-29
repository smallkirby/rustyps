mod argparser;
mod display;
mod readproc;

use simple_logger::SimpleLogger;

#[derive(Debug)]
struct Ps {
  parser: argparser::PsParser,
}

impl Ps {
  fn new() -> Ps {
    Ps {
      parser: argparser::PsParser::from(std::env::args()),
    }
  }
  pub fn run(&mut self) -> i32 {
    self.arg_check_conflicts();

    log::trace!("===== ps output follows ====");
    self.init_output();
    self.lists_and_needs();
    display::simple_spew();
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
    .with_level(log::LevelFilter::Trace)
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
