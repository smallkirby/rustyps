#[derive(Debug, PartialEq)]
pub enum ArgType {
    ArgGnu,
    ArgPgrp,
    ArgSysv,
    ArgPid,
    ArgBsd,
    ArgFail,
    ArgSess,
}

pub fn arg_type(arg: &String) -> ArgType {
    let c0 = arg.chars().nth(0);
    match c0 {
      None => ArgType::ArgFail,
      Some(c0) => match c0 {
        'a'..='z' => ArgType::ArgBsd,
        'A'..='Z' => ArgType::ArgBsd,
        '0'..='9' => ArgType::ArgPid,
        '+' => ArgType::ArgSess,
        '-' => {
          let c1 = arg.chars().nth(1);
          match c1 {
            None => ArgType::ArgFail,
            Some(c1) => match c1 {
              'a'..='z' => ArgType::ArgSysv,
              'A'..='Z' => ArgType::ArgSysv,
              '0'..='9' => ArgType::ArgPgrp,
              '-' => {
                let c2 = arg.chars().nth(2);
                match c2 {
                  None => ArgType::ArgFail,
                  Some(c2) => match c2 {
                    'a'..='z' => ArgType::ArgGnu,
                    'A'..='Z' => ArgType::ArgGnu,
                    _ => ArgType::ArgFail,
                  }
                }
              },
              _ => ArgType::ArgFail,
            }
          }
        },
        _ => ArgType::ArgFail,
      }
    }
}

#[cfg(test)]
mod tests {
  #[test]
  fn arg_type_parser() {
    assert_eq!(super::arg_type(&String::from("")), super::ArgType::ArgFail);
    assert_eq!(super::arg_type(&String::from("?")), super::ArgType::ArgFail);
    assert_eq!(super::arg_type(&String::from("ax")), super::ArgType::ArgBsd);
    assert_eq!(super::arg_type(&String::from("Ax")), super::ArgType::ArgBsd);
    assert_eq!(super::arg_type(&String::from("3")), super::ArgType::ArgPid);
    assert_eq!(super::arg_type(&String::from("+3")), super::ArgType::ArgSess);
    assert_eq!(super::arg_type(&String::from("-3")), super::ArgType::ArgPgrp);
    assert_eq!(super::arg_type(&String::from("-ax")), super::ArgType::ArgSysv);
    assert_eq!(super::arg_type(&String::from("-")), super::ArgType::ArgFail);
    assert_eq!(super::arg_type(&String::from("- ")), super::ArgType::ArgFail);
    assert_eq!(super::arg_type(&String::from("--ax")), super::ArgType::ArgGnu);
    assert_eq!(super::arg_type(&String::from("--AX")), super::ArgType::ArgGnu);
    assert_eq!(super::arg_type(&String::from("---")), super::ArgType::ArgFail);
    assert_eq!(super::arg_type(&String::from("--")), super::ArgType::ArgFail);
    assert_eq!(super::arg_type(&String::from("--3")), super::ArgType::ArgFail);
  }
}