use crate::argparser;
use crate::readproc;

pub fn simple_spew(parser: &argparser::PsParser) -> Result<(), String> {
  let mut pt = match readproc::openproc(readproc::PROC_FILLSTAT, None, None) {
    Ok(_pt) => _pt, 
    Err(msg) => return Err(msg),
  };
  log::trace!("simple_spew: opened PROCTAB");

  // display
  loop {
    if let Some(p) = readproc::readproc(&mut pt) {
      log::trace!("success readproc");
      if readproc::want_this_proc(&p, &parser) {
        show_one_proc(&p);
      }
    } else {
      log::trace!("fail readproc");
      break;
    }
  }
  Ok(())
}

// XXX 
pub fn show_one_proc(p: &readproc::PROCT) {
  log::trace!("show_one_proc()");
  println!("{}: {}", p.cmd, p.tgid);
}

