use std::convert::TryInto;
use std::io::Read;
use std::os;
use std::os::unix::fs::MetadataExt;
use std::path;

use crate::argparser;

type pid_t = i32;
type uid_t = i32;

pub const PROC_FILLMEM: u64 = 0x1;
pub const PROC_FILLCOM: u64 = 0x2;
pub const PROC_FILLENV: u64 = 0x4;
pub const PROC_FILLUSER: u64 = 0x8;

pub const PROC_FILLGRP: u64 = 0x10;
pub const PROC_FILLSTATUS: u64 = 0x20;
pub const PROC_FILLSTAT: u64 = 0x40;
pub const PROC_FILLARG: u64 = 0x100;
pub const PROC_FILLCGROUP: u64 = 0x200;

pub const PROC_PID: u64 = 0x1000;
pub const PROC_UID: u64 = 0x4000;

pub const PROC_FILLNS: u64 = 0x8000;
pub const PROC_FILLSYSTEMD: u64 = 0x80000;
pub const PROC_FILL_LXC: u64 = 0x80000;

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
  pub tgid: i32, // thread group ID
  pub tid: i32,  // thread ID
  pub pathname: String,
  pub euid: u32, // effective uid
  pub egid: u32, // effective gid
  pub state: ProcState,
  pub ppid: u32,
  pub pgrp: u32,
  pub session: u32,
  pub tty: u32,
  pub tpgid: u32,
  pub flags: u64,
  pub min_flt: u64,
  pub cmin_flt: u64,
  pub maj_flt: u64,
  pub cmaj_flt: u64,
  pub utime: u64,
  pub stime: u64,
  pub cutime: u64,
  pub cstime: u64,
  pub priority: u64,
  pub nice: u64,
  pub nlwp: u64,
  pub alarm: u64,
  pub start_time: u64,
  pub vsize: u64,
  pub rss: u64,
  pub rss_rlim: u64,
  pub start_code: u64,
  pub end_code: u64,
  pub start_stack: u64,
  pub kstk_esp: u64,
  pub kstk_eip: u64,
  pub wchan: u64,
  pub exit_signal: u64,
  pub processor: u64,
  pub rtprio: u64,
  pub sched: u64,
  pub cmd: String,
}

#[derive(Default)]
pub struct PROCTAB {
  pub procfs: Option<std::fs::ReadDir>,
  pub taskdir: Option<std::fs::ReadDir>,
  pub taskdir_user: i64,
  pub finder: Option<fn(&mut PROCTAB) -> Option<PROCT>>,
  pub reader: Option<fn(&PROCTAB, &mut PROCT) -> Option<()>>,
  pub taskfinder: Option<fn(&PROCTAB, &PROCT, &PROCT, &String) -> i32>,
  pub taskreader: Option<fn(&PROCTAB, &PROCT, &PROCT, &String) -> Option<PROCT>>,
  pub pids: Vec<i32>,
  pub uids: Vec<i32>,
  pub nuid: i32,
  pub i: i32,
  pub flags: u64,
  pub u: u32,
  pub path: std::path::PathBuf,
  pub pathlen: u32,
}

pub fn openproc(
  flags: u64,
  pidlist: Option<Vec<pid_t>>,
  uidlist: Option<Vec<uid_t>>,
) -> Result<PROCTAB, String> {
  let mut pt = PROCTAB {
    flags: PROC_FILLSTAT,
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
    log::trace!("assign finder to PROCTAB");
  }
  pt.flags = flags;

  return Ok(pt);
}

pub fn readproc(pt: &mut PROCTAB) -> Option<PROCT> {
  log::trace!("readproc()");

  loop {
    let mut p = match pt.finder.unwrap()(pt) {
      Some(_p) => _p,
      None => {
        log::trace!("failed to find next pid");
        return None;
      }
    };
    match pt.reader.unwrap()(&pt, &mut p) {
      Some(()) => {
        log::trace!("success read proc: {:?}", p);
        return Some(p);
      }
      None => {
        log::trace!("failed pt.reader()");
        continue;
      }
    }
  }
}

pub fn want_this_proc(p: &PROCT, parser: &argparser::PsParser) -> bool {
  log::trace!("want_this_proc(): {:?}", p);

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
  log::trace!("pt.path: {:?}", pt.path);
  let sb = match std::fs::metadata(&pt.path) {
    Ok(meta) => meta,
    Err(_) => return None,
  };

  p.euid = sb.uid();
  p.egid = sb.gid();

  if pt.flags & PROC_FILLSTAT != 0 {
    let statpath = path::PathBuf::from(format!("{}/stat", pt.path.to_str().unwrap()));
    let mut statfile = std::fs::File::open(statpath.to_str().unwrap()).unwrap();
    let mut stat = String::new();
    match statfile.read_to_string(&mut stat) {
      Ok(n) => {
        log::trace!("read stat: {} bytes", n);
      }
      Err(_) => log::error!("failed to read stat"),
    };
    match stat2proc(&stat, p) {
      Ok(()) => log::trace!("success stat2proc()"),
      Err(_msg) => return None,
    }
  };

  if pt.flags & PROC_FILLMEM != 0 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLMEM != 0 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLUSER != 0 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLGRP != 0 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLENV != 0 {
    unimplemented!();
  };

  if (pt.flags & PROC_FILLARG != 0) && (pt.flags & PROC_FILLCOM != 0) {
    unimplemented!();
  };

  if pt.flags & PROC_FILLCGROUP != 0 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLCOM != 0 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLNS != 0 {
    unimplemented!();
  };

  if pt.flags & PROC_FILLSYSTEMD != 0 {
    unimplemented!();
  };

  if pt.flags & PROC_FILL_LXC != 0 {
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
    match d.file_name().to_str().unwrap().parse::<i32>() {
      Ok(n) => {
        log::trace!("success parse proc name: {:?}", n);
        pt.path = path::PathBuf::from(String::from(format!(
          "/proc/{}",
          d.file_name().to_str().unwrap()
        )));
        return Some(PROCT {
          tgid: n,
          tid: n,
          ..Default::default()
        });
      }
      Err(_) => {
        log::trace!(
          "failed to parse proc name: {:?}",
          d.file_name().to_str().unwrap()
        );
        continue;
      }
    };
  }
  unreachable!();
}

// XXX
fn simple_readtask(pt: &PROCTAB, p: &PROCT, t: &PROCT, path: &String) -> Option<PROCT> {
  unimplemented!();
}

pub fn i2u64(n: i64) -> u64 {
  if n > 0 {
    n.try_into().unwrap()
  } else {
    (-n).try_into().unwrap()
  }
}
pub fn i2u32(n: i32) -> u32 {
  if n > 0 {
    n.try_into().unwrap()
  } else {
    (-n).try_into().unwrap()
  }
}

pub fn stat2proc(s: &String, p: &mut PROCT) -> Result<(), String> {
  /* sample stat output is below:
    $ cat /proc/$$/stat
    1504081 (bash) S 3423 1504081 1504081 34984 1504155 4194304 2107 9649 0 0 4 0 4 5 20 0 1 0 76785102 13455360 1554 18446744073709551615 94220315791360 94220316514053 140735920746448 0 0 0 65536 3670020 1266777851 1 0 0 17 3 0 0 0 0 0 94220316744944 94220316792324 94220326436864 140735920752777 140735920752791 140735920752791 140735920754666 0
  */
  log::trace!("{:?}", s);
  //macro_rules! i2u {
  //  ($x:expr, i64) => {
  //    if $x < 0 {
  //      (-$x).parse::<u64>().unwrap()
  //    } else {
  //      $x.parse::u64().unwrap()
  //    }
  //  };
  //  ($x:expr, i32) => {
  //    if $x < 0 {
  //      (-$x).parse::<u32>().unwrap()
  //    } else {
  //      $x.parse::u32().unwrap()
  //    }
  //  };
  //}
  scan_fmt!("1406 (sd-pam) S 1405 1405 1405 0 -1 1077936448 48 0 0 0 0 0 0 0 20 0 1 0 910 173793280 845 18446744073709551615 1 1 0 0 0 0 0 4096 0 0 0 0 17 3 0 0 0 0 0 0 0 0 0 0 0 0 0\n", 
  "{} ({}) {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n",
    i64, String, String, i32, i32, i32, i32, i32, i64, i64,i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, u64, u64, u64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64
  ).unwrap();
  let (
    pid, com, state, ppid, pgrp, sess, ttynr, tpgid, flags, minflt, cminflt, majflt, cmajflt, utime, stime, cutime, cstime, prio, nice, num_threads, itrealvalue, starttime, vsize, rss, rsslim, startcode, endcode, startstack, kstkesp, kstkeip, signal, blocked, sigignore, sigcatch, wchan, nswap, cnswap, exit_signal, processor, rt_prio,policy, delayacct_blkio_ticks, guest_time, cguest_time, start_data, end_data, start_brk, arg_start, arg_end, env_start,env_end, exit_code,
  ) = if let Ok(r) = scan_fmt!(s, "{} ({}) {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n",
    i64, String, String, i32, i32, i32, i32, i32, i64, i64,i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, u64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64
  ) {
    r
  } else {
    scan_fmt!(s, "{} (({})) {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n",
      i64, String, String, i32, i32, i32, i32, i32, i64, i64,i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, u64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64
    ).unwrap()
  };

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
  p.ppid = i2u32(ppid);
  p.pgrp = i2u32(pgrp);
  p.session = i2u32(sess);
  p.tty = i2u32(ttynr);
  p.tpgid = i2u32(tpgid);
  p.flags = i2u64(flags);
  p.min_flt = i2u64(minflt);
  p.cmin_flt = i2u64(cminflt);
  p.maj_flt = i2u64(majflt);
  p.cmaj_flt = i2u64(cmajflt);
  p.utime = i2u64(utime);
  p.stime = i2u64(stime);
  p.cutime = i2u64(cutime);
  p.cstime = i2u64(cstime);
  p.priority = i2u64(prio);
  p.nice = i2u64(nice);
  p.nlwp = i2u64(num_threads);
  p.alarm = i2u64(itrealvalue);
  p.start_time = i2u64(stime);
  p.vsize = i2u64(vsize);
  p.rss = i2u64(rss);
  p.rss_rlim = rsslim;
  p.start_code = i2u64(startcode);
  p.end_code = i2u64(endcode);
  p.start_stack = i2u64(startstack);
  p.kstk_esp = i2u64(kstkesp);
  p.kstk_eip = i2u64(kstkeip);
  p.wchan = i2u64(wchan);
  p.exit_signal = i2u64(exit_signal);
  p.processor = i2u64(processor);
  p.rtprio = i2u64(rt_prio);
  p.sched = i2u64(policy);
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
