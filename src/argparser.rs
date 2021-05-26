#[derive(Debug)]
pub struct PsParser{
  args: Vec<String>,
  curargix: usize,
}

#[derive(Debug)]
pub enum SelectionNode {
  PID(PidSelection),
}

#[derive(Debug)]
struct PidSelection {
  pid: u64,
}

#[derive(Debug)]
pub enum ArgType {
  GNU,
  SYSV,
  BSD,
  PGRP,
  SESS,
  PID,
  FAIL,
}

impl PsParser {
  pub fn from(args: std::env::Args) -> PsParser {
    PsParser{curargix: 0, args: args.collect()}
  }

  // main function of parser
  pub fn parse(mut self) -> Result<Vec<SelectionNode>, String> {
    let option_nodes = self.parse_all_options();
    return Err(String::from("not imp"));
  }

  // parse all options and get list of SelectionNode. can be called only once.
  pub fn parse_all_options(&mut self) -> Result<Vec<SelectionNode>, String> {
    self.curargix = 1;
    while self.curargix < self.args.len() {
      log::trace!("arg: {}", self.args[self.curargix]);

      match arg_type(&self.args[self.curargix]) {
        ArgType::GNU  =>  {
          log::trace!("GNU type arg: {}", &self.args[self.curargix]);
          self.parse_gnu_option();
        },
        _ => {
          unimplemented!();
        }
      };

      self.curargix += 1;
    }
    return Err(String::from("not imp"));
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
    let argname = &arg[2..std::cmp::min(p0,p1)];
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

    return Err(String::from("not imp"))
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
    if std::cmp::min(p0, p1) >= arg.len()-1 { // arg value should be in next arg
      self.curargix += 1;
      if self.args.len() <= self.curargix {
        return None;
      } else {
        return Some(self.args[self.curargix].clone());
      }
    } else {
      return Some(String::from(&self.args[self.curargix][std::cmp::min(p0,p1)+1..]));
    }
  }

  pub fn parse_list(&mut self, argval: &String, f: fn(&Vec<String>) -> Option<Vec<SelectionNode>>) -> Option<Vec<SelectionNode>> {
    let mut need_item = true;
    let mut items = 0;
    // count items
    for c in argval.chars() {
      match c {
        ' ' | ',' | '\t' => if need_item {
          return None;
        } else {
          need_item = true;
        },
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
    let vals: Vec<String> = argval.split(&[',',' ','\t'][..]).map(|x| x.into()).collect();
    return f(&vals);
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
                  }
                }
              },
              _ => ArgType::FAIL,
            }
          }
        },
        _ => ArgType::FAIL,
      }
    }
}

pub fn strpbrk(msg: &String, delims: &String) -> Option<usize> {
  for (ix,c) in msg.chars().enumerate() {
    if let Some(_) = delims.find(c) {
      return Some(ix);
    }
  }
  None
}

pub fn parse_pid(vals: &Vec<String>) -> Option<Vec<SelectionNode>> {
  let mut selection_list: Vec<SelectionNode> = vec![];
  for val in vals {
    let n = match val.parse::<u64>() {
      Ok(_n) => _n,
      Err(_) => return None,
    };
    selection_list.push(SelectionNode::PID (
      PidSelection{pid: n},
    ));
  }
  return Some(selection_list);
}