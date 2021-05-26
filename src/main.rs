mod argparser;

use simple_logger::SimpleLogger;
use std::env;

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Trace)
        .init()
        .unwrap();

    // XXX have to register atexit func?
    /* */

    let myname = std::env::current_exe();
    log::trace!("{:?}", myname);

    // XXX must set sighandlers
    /* */
    argparser::arg_parse(env::args().collect());
}

pub fn do_help(opt: String, rc: i32) {
    log::trace!("do_help called with opt:{}, rc:{}", opt, rc);
    std::process::exit(rc);
}
