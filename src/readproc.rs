#[derive(Debug, Default, PartialEq)]
pub struct PROCT {}

#[derive(Default)]
pub struct PROCTAB {
  procfs: Option<std::fs::ReadDir>,
  taskdir: Option<std::fs::ReadDir>,
  taskdir_user: i64,
  finder: Option<fn(&PROCTAB, &PROCT) -> i32>,
  reader: Option<fn(&PROCTAB, &PROCT) -> Option<PROCT>>,
  taskfinder: Option<fn(&PROCTAB, &PROCT, &PROCT, &String) -> i32>,
  taskreader: Option<fn(&PROCTAB, &PROCT, &PROCT, &String) -> Option<PROCT>>,
  pids: Vec<i32>,
  uids: Vec<i32>,
  nuid: i32,
  i: i32,
  flags: u32,
  u: u32,
  path: std::path::PathBuf,
  pathlen: u32,
}

pub fn openproc(flags: u64) -> Result<PROCTAB, String> {
  let mut pt = PROCTAB {
    ..Default::default()
  };

  pt.taskdir = None;
  pt.taskdir_user = -1;
  pt.taskfinder = Some(simple_nexttid);
  pt.taskreader = Some(simple_readtask);
  pt.reader = Some(simple_readproc);

  unimplemented!();

  return Err(String::from("not impl openproc"));
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
fn simple_readtask(pt: &PROCTAB, p: &PROCT, t: &PROCT, path: &String) -> Option<PROCT> {
  unimplemented!();
  return None;
}
