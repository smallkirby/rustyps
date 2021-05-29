mod argparser;

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
    fn run(self) -> i32 {
        0
    }
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
    let ps = Ps::new();
    ps.run();
}

pub fn do_help(opt: String, rc: i32) {
    log::trace!("do_help called with opt:{}, rc:{}", opt, rc);
    std::process::exit(rc);
}
