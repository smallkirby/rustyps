#[derive(Debug)]
pub struct PsParser {
  pub args: Vec<String>,
  pub curargix: usize,
  pub thread_flags: Vec<ThreadFlag>,
  pub select_bits: u32,
  pub simple_select: bool,
  pub screen_cols: u32,
  pub w_count: u32,
  pub all_process: bool,
  pub selection_list: Vec<SelectionNode>,
}

impl Default for PsParser {
  fn default() -> Self {
    Self {
      args: vec![],
      curargix: 0,
      thread_flags: vec![],
      select_bits: 0,
      simple_select: false,
      screen_cols: 0,
      w_count: 211,
      all_process: false,
      selection_list: vec![],
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum SelectionNode {
  PID(PidSelection),
}

#[derive(Debug, PartialEq)]
pub struct PidSelection {
  pub pid: Vec<i32>,
}

#[derive(Debug, PartialEq)]
pub enum ArgType {
  GNU,
  SYSV,
  BSD,
  PGRP,
  SESS,
  PID,
  FAIL,
}

#[derive(Debug, PartialEq)]
pub enum ThreadFlag {
  B_H,
  B_m,
  U_m,
  U_T,
  U_L,
  SHOW_PROC,
  SHOW_TASK,
  SHOW_BOTH,
  LOOSE_TASKS,
  NO_SORT,
  NO_FOREST,
  MUST_USE,
}

impl PsParser {
  pub fn from(args: std::env::Args) -> PsParser {
    PsParser {
      curargix: 0,
      args: args.collect(),
      thread_flags: vec![],
      ..Default::default()
    }
  }

  // main function of parser
  // replacement of 'arg_parse()
  pub fn parse(mut self) -> Result<Vec<SelectionNode>, String> {
    match self.arg_parse() {
      Ok(list) => Ok(list),
      Err(msg) => Err(msg),
    }
  }

  pub fn arg_parse(&mut self) -> Result<Vec<SelectionNode>, String> {
    let option_nodes = match self.parse_all_options() {
      Ok(list) => list,
      Err(msg) => return Err(msg),
    };
    match self.thread_option_check() {
      Ok(_) => log::trace!("thread option updated"),
      Err(msg) => return Err(msg),
    };
    match self.process_sf_options() {
      Ok(_) => log::trace!("process sf option updated"),
      Err(msg) => return Err(msg),
    };
    match self.choose_dimensions() {
      Ok(_) => log::trace!("dimensions updated"),
      Err(msg) => return Err(msg),
    };
    return Ok(option_nodes);
  }

  // parse all options and get list of SelectionNode. can be called only once.
  pub fn parse_all_options(&mut self) -> Result<Vec<SelectionNode>, String> {
    let mut selection_list: Vec<SelectionNode> = vec![];
    self.curargix = 1;
    while self.curargix < self.args.len() {
      log::trace!("arg: {}", self.args[self.curargix]);

      match arg_type(&self.args[self.curargix]) {
        ArgType::GNU => {
          log::trace!("GNU type arg: {}", &self.args[self.curargix]);
          match self.parse_gnu_option() {
            Ok(mut list) => selection_list.append(&mut list),
            Err(msg) => return Err(msg),
          }
        }
        _ => {
          unimplemented!();
        }
      };

      self.curargix += 1;
    }
    return Ok(selection_list);
  }

  // parse GNU options.
  // @self.args[@self.curargix] should be start with "--"
  pub fn parse_gnu_option(&mut self) -> Result<Vec<SelectionNode>, String> {
    let mut selection_list: Vec<SelectionNode> = vec![];
    let arg = &self.args[self.curargix];
    // find first appearance of delimiter(:=)
    let p0 = match arg.find('=') {
      Some(i) => i,
      None => arg.len(),
    };
    let p1 = match arg.find(':') {
      Some(i) => i,
      None => arg.len(),
    };
    let argname = &arg[2..std::cmp::min(p0, p1)];
    log::trace!("GNU arg name: {}", argname);

    if argname == "pid" {
      log::trace!("processing GNU --pid");
      let arg = match self.grab_gnu_arg() {
        Some(s) => s,
        None => return Err(String::from("pid specification is invalid.")),
      };
      log::trace!("GNU --pid value is {}", arg);
      match self.parse_list(&arg, parse_pid) {
        Some(mut list) => selection_list.append(&mut list),
        None => return Err(String::from("error parse_pid")),
      }
      log::trace!("GNU pid parsed: {:?}", selection_list);
    } else {
      return Err(String::from("unknown gnu long option"));
    }

    return Ok(selection_list);
  }

  // get GNU type arg value
  pub fn grab_gnu_arg(&mut self) -> Option<String> {
    let arg = &self.args[self.curargix];
    let p0 = match arg.find('=') {
      Some(i) => i,
      None => arg.len(),
    };
    let p1 = match arg.find(':') {
      Some(i) => i,
      None => arg.len(),
    };
    if std::cmp::min(p0, p1) >= arg.len() - 1 {
      // arg value should be in next arg
      self.curargix += 1;
      if self.args.len() <= self.curargix {
        return None;
      } else {
        return Some(self.args[self.curargix].clone());
      }
    } else {
      return Some(String::from(
        &self.args[self.curargix][std::cmp::min(p0, p1) + 1..],
      ));
    }
  }

  pub fn parse_list(
    &mut self,
    argval: &String,
    f: fn(&Vec<String>) -> Option<Vec<SelectionNode>>,
  ) -> Option<Vec<SelectionNode>> {
    let mut need_item = true;
    let mut items = 0;
    // count items
    for c in argval.chars() {
      match c {
        ' ' | ',' | '\t' => {
          if need_item {
            return None;
          } else {
            need_item = true;
          }
        }
        _ => {
          if need_item {
            items += 1;
          }
          need_item = false;
        }
      }
    }
    log::trace!("items: {}", items);

    // parse each items
    let vals: Vec<String> = argval
      .split(&[',', ' ', '\t'][..])
      .map(|x| x.into())
      .collect();
    return f(&vals);
  }

  // XXX
  pub fn thread_option_check(&mut self) -> Result<(), String> {
    if self.thread_flags.len() == 0 {
      self.thread_flags.push(ThreadFlag::SHOW_PROC);
      return Ok(());
    }
    return Err(String::from("thread option: not imp"));
  }

  // XXX
  pub fn process_sf_options(&mut self) -> Result<(), String> {
    Ok(())
    //if self.simple_select {
    //  self.select_bits = 0xaa00;
    //  return Ok(());
    //}
    //return Err(String::from("thread option: not imp"));
  }

  pub fn choose_dimensions(&mut self) -> Result<(), String> {
    if self.w_count != 1 && (self.screen_cols < 132) {
      self.screen_cols = 132;
    }
    if self.w_count > 1 {
      self.screen_cols = 2 * 64 * 1024 * 1;
    }
    return Ok(());
  }
}

pub fn arg_type(arg: &String) -> ArgType {
  let c0 = arg.chars().nth(0);
  match c0 {
    None => ArgType::FAIL,
    Some(c0) => match c0 {
      'a'..='z' => ArgType::BSD,
      'A'..='Z' => ArgType::BSD,
      '0'..='9' => ArgType::PID,
      '+' => ArgType::SESS,
      '-' => {
        let c1 = arg.chars().nth(1);
        match c1 {
          None => ArgType::FAIL,
          Some(c1) => match c1 {
            'a'..='z' => ArgType::SYSV,
            'A'..='Z' => ArgType::SYSV,
            '0'..='9' => ArgType::PGRP,
            '-' => {
              let c2 = arg.chars().nth(2);
              match c2 {
                None => ArgType::FAIL,
                Some(c2) => match c2 {
                  'a'..='z' => ArgType::GNU,
                  'A'..='Z' => ArgType::GNU,
                  _ => ArgType::FAIL,
                },
              }
            }
            _ => ArgType::FAIL,
          },
        }
      }
      _ => ArgType::FAIL,
    },
  }
}

pub fn parse_pid(vals: &Vec<String>) -> Option<Vec<SelectionNode>> {
  let mut selection_list: Vec<SelectionNode> = vec![];
  for val in vals {
    let n = match val.parse::<i32>() {
      // XXX should be able to parse 1,2,3
      Ok(_n) => _n,
      Err(_) => return None,
    };
    selection_list.push(SelectionNode::PID(PidSelection { pid: vec![n] }));
  }
  return Some(selection_list);
}

#[cfg(test)]
mod tests {
  #[test]
  fn parser_gnu_pid() {
    let parser0 = super::PsParser {
      args: vec![String::from("me"), String::from("--pid=33")],
      curargix: 0,
      thread_flags: vec![],
      ..Default::default()
    };
    let parser1 = super::PsParser {
      args: vec![String::from("me"), String::from("--pid=33,44,55")],
      curargix: 0,
      thread_flags: vec![],
      ..Default::default()
    };
    let parser2 = super::PsParser {
      args: vec![String::from("me"), String::from("--pid:33,44,55")],
      curargix: 0,
      thread_flags: vec![],
      ..Default::default()
    };
    let parser3 = super::PsParser {
      args: vec![
        String::from("me"),
        String::from("--pid"),
        String::from("33,44,55"),
      ],
      curargix: 0,
      thread_flags: vec![],
      ..Default::default()
    };
    let b0 = vec![super::SelectionNode::PID(super::PidSelection {
      pid: vec![33],
    })];
    let b1 = vec![
      super::SelectionNode::PID(super::PidSelection { pid: vec![33] }),
      super::SelectionNode::PID(super::PidSelection { pid: vec![44] }),
      super::SelectionNode::PID(super::PidSelection { pid: vec![55] }),
    ];
    assert_eq!(parser0.parse().unwrap(), b0);
    assert_eq!(parser1.parse().unwrap(), b1);
    assert_eq!(parser2.parse().unwrap(), b1);
    assert_eq!(parser3.parse().unwrap(), b1);
  }
}
