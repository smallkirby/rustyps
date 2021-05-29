use std::convert::TryInto;
use std::os;
use std::os::unix::fs::MetadataExt;
use std::path;

use crate::argparser;

type pid_t = i32;
type uid_t = i32;

const PROC_FILLMEM: u64 = 0x1;
const PROC_FILLCOM: u64 = 0x2;
const PROC_FILLENV: u64 = 0x4;
const PROC_FILLUSER: u64 = 0x8;

const PROC_FILLGRP: u64 = 0x10;
const PROC_FILLSTATUS: u64 = 0x20;
const PROC_FILLSTAT: u64 = 0x40;
const PROC_FILLARG: u64 = 0x100;
const PROC_FILLCGROUP: u64 = 0x200;

const PROC_PID: u64 = 0x1000;
const PROC_UID: u64 = 0x4000;

const PROC_FILLNS: u64 = 0x8000;
const PROC_FILLSYSTEMD: u64 = 0x80000;
const PROC_FILL_LXC: u64 = 0x80000;

#[derive(Debug, PartialEq)]
pub enum ProcState {
  RUNNING,
  SLEEPING,
  WAITING,
  ZOMBIE,
  STOPPED,
  TSTOP,
  PAGING,
  DEAD,
  DEAD2,
  WAKEKILL,
  WAKING,
  PARKED,
  UNKNOWN,
}
impl Default for ProcState {
  fn default() -> Self {
    ProcState::UNKNOWN
  }
}

#[derive(Debug, Default, PartialEq)]
pub struct PROCT {
  // all the information about proc
  tgid: i32, // thread group ID
  tid: i32,  // thread ID
  pathname: String,
  euid: u32, // effective uid
  egid: u32, // effective gid
  state: ProcState,
  ppid: u32,
  pgrp: u32,
  session: u32,
  tty: u32,
  tpgid: u32,
  flags: u64,
  min_flt: u64,
  cmin_flt: u64,
  maj_flt: u64,
  cmaj_flt: u64,
  utime: u64,
  stime: u64,
  cutime: u64,
  cstime: u64,
  priority: u64,
  nice: u64,
  nlwp: u64,
  alarm: u64,
  start_time: u64,
  vsize: u64,
  rss: u64,
  rss_rlim: u64,
  start_code: u64,
  end_code: u64,
  start_stack: u64,
  kstk_esp: u64,
  kstk_eip: u64,
  wchan: u64,
  exit_signal: u64,
  processor: u64,
  rtprio: u64,
  sched: u64,
  cmd: String,
}

#[derive(Default)]
pub struct PROCTAB {
  procfs: Option<std::fs::ReadDir>,
  taskdir: Option<std::fs::ReadDir>,
  taskdir_user: i64,
  finder: Option<fn(&mut PROCTAB) -> Option<PROCT>>,
  reader: Option<fn(&PROCTAB, &mut PROCT) -> Option<()>>,
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
    let mut p = match pt.finder.unwrap()(pt) {
      Some(_p) => _p,
      None => return None,
    };
    match pt.reader.unwrap()(&pt, &mut p) {
      Some(()) => return Some(p),
      None => continue,
    }
  }
}

pub fn want_this_proc(p: &PROCT, parser: &argparser::PsParser) -> bool {
  let mut proc_is_wanted = false;
  if !parser.all_process {
    // use table for -a a d g x
    if parser.simple_select || parser.selection_list.len() == 0 {
      if table_accept() {
        unimplemented!();
      }
    } else {
      // search lists
      if proc_was_listed(&p, &parser) {
        proc_is_wanted = true;
      }
    }
  }
  // finish
  return proc_is_wanted;
}

// return None if the proc file does no more exist.
fn simple_readproc(pt: &PROCTAB, p: &mut PROCT) -> Option<()> {
  let sb = match std::fs::metadata(&pt.path) {
    Ok(meta) => meta,
    Err(_) => return None,
  };

  p.euid = sb.uid();
  p.egid = sb.gid();

  if pt.flags & PROC_FILLSTAT != 1 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLMEM != 1 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLMEM != 1 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLUSER != 1 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLGRP != 1 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLENV != 1 {
    unimplemented!();
  };

  if (pt.flags & PROC_FILLARG != 1) && (pt.flags & PROC_FILLCOM != 1) {
    unimplemented!();
  };

  if pt.flags & PROC_FILLCGROUP != 1 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLCOM != 1 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLNS != 1 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLSYSTEMD != 1 {
    unimplemented!();
  };

  if pt.flags & PROC_FILL_LXC != 1 {
    unimplemented!();
  };

  return Some(());
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
          ..Default::default()
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
}

pub fn stat2proc(s: &String, p: &mut PROCT) -> Result<(), String> {
  /* sample stat output is below:
    $ cat /proc/$$/stat
    1504081 (bash) S 3423 1504081 1504081 34984 1504155 4194304 2107 9649 0 0 4 0 4 5 20 0 1 0 76785102 13455360 1554 18446744073709551615 94220315791360 94220316514053 140735920746448 0 0 0 65536 3670020 1266777851 1 0 0 17 3 0 0 0 0 0 94220316744944 94220316792324 94220326436864 140735920752777 140735920752791 140735920752791 140735920754666 0
  */
  let (
    pid, com, state, ppid, pgrp, sess, ttynr, tpgid, flags, minflt, cminflt, majflt, cmajflt, utime, stime, cutime, cstime, prio, nice, num_threads, itrealvalue, starttime, vsize, rss, rsslim, startcode, endcode, startstack, kstkesp, kstkeip, signal, blocked, sigignore, sigcatch, wchan, nswap, cnswap, exit_signal, processor, rt_prio,policy, delayacct_blkio_ticks, guest_time, cguest_time, start_data, end_data, start_brk, arg_start, arg_end, env_start,env_end, exit_code,
  ): (
    u64, String, String, u32, u32, u32, u32, u32, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64,
  ) = scan_fmt!(s, "{} ({}) {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
    u64, String, String, u32, u32, u32, u32, u32, u64, u64,u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64
  ).unwrap();

  p.state = match state.as_str() {
    "R" => ProcState::RUNNING,
    "S" => ProcState::SLEEPING,
    "D" => ProcState::WAITING,
    "Z" => ProcState::ZOMBIE,
    "T" => ProcState::STOPPED,
    "t" => ProcState::TSTOP,
    "W" => ProcState::PAGING,
    "X" => ProcState::DEAD,
    "x" => ProcState::DEAD2,
    "K" => ProcState::WAKEKILL,
    "W" => ProcState::WAKING,
    "P" => ProcState::PARKED,
    _ => ProcState::UNKNOWN,
  };
  p.ppid = ppid;
  p.pgrp = pgrp;
  p.session = sess;
  p.tty = ttynr;
  p.tpgid = tpgid;
  p.flags = flags;
  p.min_flt = minflt;
  p.cmin_flt = cminflt;
  p.maj_flt = majflt;
  p.cmaj_flt = cmajflt;
  p.utime = utime;
  p.stime = stime;
  p.cutime = cutime;
  p.cstime = cstime;
  p.priority = prio;
  p.nice = nice;
  p.nlwp = num_threads;
  p.alarm = itrealvalue;
  p.start_time = stime;
  p.vsize = vsize;
  p.rss = rss;
  p.rss_rlim = rsslim;
  p.start_code = startcode;
  p.end_code = endcode;
  p.start_stack = startstack;
  p.kstk_esp = kstkesp;
  p.kstk_eip = kstkeip;
  p.wchan = wchan;
  p.exit_signal = exit_signal;
  p.processor = processor;
  p.rtprio = rt_prio;
  p.sched = policy;
  p.cmd = com;
  Ok(())
}

pub fn table_accept() -> bool {
  unimplemented!();
}

pub fn proc_was_listed(p: &PROCT, parser: &argparser::PsParser) -> bool {
  let sn = &parser.selection_list;
  if sn.len() == 0 {
    false
  } else {
    for snode in sn {
      match snode {
        argparser::SelectionNode::PID(pid_selection) => {
          for pid in pid_selection.pid.iter() {
            if p.tgid == *pid {
              return true;
            }
          }
        }
        _ => {
          unimplemented!();
        }
      }
    }
    false
  }
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

  #[test]
  fn simple_nextpid_iterate_all() {
    let mut pt = super::openproc(0, None, None).unwrap();
    let mut count = 0;
    loop {
      match super::simple_nextpid(&mut pt) {
        Some(p) => count += 1,
        None => break,
      };
    }
    assert_eq!(count > 1, true);
  }

  #[test]
  fn test_stat2proc() {
    let stat = String::from("1504081 (bash) S 3423 1504081 1504081 34984 1504155 4194304 2107 9649 0 0 4 0 4 5 20 0 1 0 76785102 13455360 1554 18446744073709551615 94220315791360 94220316514053 140735920746448 0 0 0 65536 3670020 1266777851 1 0 0 17 3 0 0 0 0 0 94220316744944 94220316792324 94220326436864 140735920752777 140735920752791 140735920752791 140735920754666 0");
    let mut p = super::PROCT {
      ..Default::default()
    };
    super::stat2proc(&stat, &mut p).unwrap();
    assert_eq!(p.cmd, "bash");
    assert_eq!(p.session, 1504081);
  }

  #[test]
  fn want_this_proc_single_pid() {
    use crate::argparser;
    let p1 = super::PROCT {
      tgid: 3,
      ..Default::default()
    };
    let p2 = super::PROCT {
      tgid: 4,
      ..Default::default()
    };
    let selection_list = vec![argparser::SelectionNode::PID(argparser::PidSelection {
      pid: vec![3],
    })];
    let psparser = argparser::PsParser {
      selection_list: selection_list,
      ..Default::default()
    };
    assert_eq!(super::want_this_proc(&p1, &psparser), true);
    assert_eq!(super::want_this_proc(&p2, &psparser), false);
  }
}
