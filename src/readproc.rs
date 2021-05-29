use std::path;

type pid_t = i32;
type uid_t = i32;

const PROC_PID: u64 = 0x1000;
const PROC_UID: u64 = 0x4000;

#[derive(Debug, Default, PartialEq)]
pub struct PROCT {
  // all the information about proc
  tgid: i32, // thread group ID
  tid: i32,  // thread ID
  pathname: String,
}

#[derive(Default)]
pub struct PROCTAB {
  procfs: Option<std::fs::ReadDir>,
  taskdir: Option<std::fs::ReadDir>,
  taskdir_user: i64,
  finder: Option<fn(&mut PROCTAB) -> Option<PROCT>>,
  reader: Option<fn(&PROCTAB, &PROCT) -> Option<PROCT>>,
  taskfinder: Option<fn(&PROCTAB, &PROCT, &PROCT, &String) -> i32>,
  taskreader: Option<fn(&PROCTAB, &PROCT, &PROCT, &String) -> Option<PROCT>>,
  pids: Vec<i32>,
  uids: Vec<i32>,
  nuid: i32,
  i: i32,
  flags: u64,
  u: u32,
  path: std::path::PathBuf,
  pathlen: u32,
}

pub fn openproc(
  flags: u64,
  pidlist: Option<Vec<pid_t>>,
  uidlist: Option<Vec<uid_t>>,
) -> Result<PROCTAB, String> {
  let mut pt = PROCTAB {
    ..Default::default()
  };

  pt.taskdir = None;
  pt.taskdir_user = -1;
  pt.taskfinder = Some(simple_nexttid);
  pt.taskreader = Some(simple_readtask);
  pt.reader = Some(simple_readproc);

  if flags & PROC_PID != 0 {
    unimplemented!();
  } else {
    pt.procfs = match std::fs::read_dir(path::Path::new("/proc")) {
      Ok(d) => Some(d),
      Err(_) => return Err(String::from("Fatal error: failed to open /proc dir.")),
    };
    pt.finder = Some(simple_nextpid);
  }
  pt.flags = flags;

  return Ok(pt);
}

pub fn readproc(pt: &mut PROCTAB) -> Option<PROCT> {
  loop {
    let p = match pt.finder.unwrap()(pt) {
      Some(_p) => _p,
      None => return None,
    };
  }
  None // XXX
}

// XXX
fn simple_readproc(pt: &PROCTAB, p: &PROCT) -> Option<PROCT> {
  unimplemented!();
  return None;
}

// XXX
fn simple_nexttid(pt: &PROCTAB, p: &PROCT, t: &PROCT, path: &String) -> i32 {
  unimplemented!();
  return 0;
}

// XXX
fn simple_nextpid(pt: &mut PROCTAB) -> Option<PROCT> {
  loop {
    let d = match pt.procfs.as_mut().unwrap().next() {
      Some(_d) => _d.unwrap(),
      None => return None,
    };
    match d.file_name().to_str().unwrap()[0..=0].parse::<i32>() {
      Ok(n) => {
        return Some(PROCT {
          tgid: n,
          tid: n,
          pathname: String::from(format!("/proc/{}", d.file_name().to_str().unwrap())),
        })
      }
      Err(_) => continue,
    };
  }
  unreachable!();
}

// XXX
fn simple_readtask(pt: &PROCTAB, p: &PROCT, t: &PROCT, path: &String) -> Option<PROCT> {
  unimplemented!();
  return None;
}

#[cfg(test)]
mod tests {
  #[test]
  fn simple_openproc_flag0() {
    let res = super::openproc(0, None, None).unwrap().procfs.unwrap();
  }

  #[test]
  fn simple_nextpid_test() {
    let mut pt = super::openproc(0, None, None).unwrap();
    assert_eq!(super::simple_nextpid(&mut pt).unwrap().tgid, 1);
    assert_eq!(super::simple_nextpid(&mut pt).unwrap().tgid, 2);
  }
}
