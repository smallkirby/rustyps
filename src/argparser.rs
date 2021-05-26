use std::collections::LinkedList;

// globals

// end globals

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

#[derive(Debug, PartialEq)]
pub struct SelectionNode {
    u: Box<SelUnion>,
    n: i32, // number of items
    typecode: SelectionListType,
}

#[derive(Debug, PartialEq)]
pub enum SelectionListType {
    SelrUid,
    SeleUid,
    SelsUid,
    SelfUid,
    SelrGid,
    SeleGid,
    SelsGid,
    SelfGid,
    SelpGrp,
    SelPid,
    SelTty,
    SelSess,
    SelComm,
    SelPpid,
    SelPidQuick,
}

#[derive(Debug, PartialEq)]
pub struct SelUnion {
    val: u64,
}

pub enum GnuTable {
    Group,
    User,
    Cols,
    Columns,
    Context,
    Comulative,
    Deselect,
    Forest,
    Format,
    Sgroup, // group
    Header,
    Headers,
    Heading,
    Headings,
    Info,
    Lines,
    NoHeader,
    NoHeaders,
    Noheader,
    Noheaders,
    NoHeading,
    NoHeadings,
    Noheading,
    Noheadings,
    Pid,
    Ppid,
    QuickPid,
    Rows,
    Sid,
    Sort,
    Tty,
    Version,
    Width,
}

impl GnuTable {
    pub fn from_str(s: &str) -> Option<GnuTable> {
        match s {
            "Group" => Some(GnuTable::Group),
            "User" => Some(GnuTable::User),
            "cols" => Some(GnuTable::Cols),
            "columns" => Some(GnuTable::Columns),
            "context" => Some(GnuTable::Context),
            "comulative" => Some(GnuTable::Comulative),
            "deselect" => Some(GnuTable::Deselect),
            "forest" => Some(GnuTable::Forest),
            "format" => Some(GnuTable::Format),
            "group" => Some(GnuTable::Sgroup),
            "header" => Some(GnuTable::Header),
            "headers" => Some(GnuTable::Headers),
            "heading" => Some(GnuTable::Heading),
            "headings" => Some(GnuTable::Headings),
            "info" => Some(GnuTable::Info),
            "lines" => Some(GnuTable::Lines),
            "no-header" => Some(GnuTable::NoHeader),
            "no-headers" => Some(GnuTable::NoHeaders),
            "noheader" => Some(GnuTable::Noheader),
            "noheaders" => Some(GnuTable::Noheaders),
            "no-heading" => Some(GnuTable::NoHeading),
            "no-headings" => Some(GnuTable::NoHeadings),
            "noheading" => Some(GnuTable::Noheading),
            "noheadings" => Some(GnuTable::Noheadings),
            "pid" => Some(GnuTable::Pid),
            "ppid" => Some(GnuTable::Ppid),
            "quick-pid" => Some(GnuTable::QuickPid),
            "rows" => Some(GnuTable::Rows),
            "sid" => Some(GnuTable::Sid),
            "sort" => Some(GnuTable::Sort),
            "tty" => Some(GnuTable::Tty),
            "version" => Some(GnuTable::Version),
            "width" => Some(GnuTable::Width),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match *self {
            GnuTable::Group => "Group",
            GnuTable::User => "user",
            GnuTable::Cols => "cols",
            GnuTable::Columns => "colmns",
            GnuTable::Context => "context",
            GnuTable::Comulative => "comulative",
            GnuTable::Deselect => "deselect",
            GnuTable::Forest => "forest",
            GnuTable::Format => "format",
            GnuTable::Sgroup => "group",
            GnuTable::Header => "header",
            GnuTable::Headers => "headers",
            GnuTable::Heading => "heading",
            GnuTable::Headings => "headings",
            GnuTable::Info => "info",
            GnuTable::Lines => "lines",
            GnuTable::NoHeader => "no-header",
            GnuTable::NoHeaders => "no-headers",
            GnuTable::Noheader => "noheader",
            GnuTable::Noheaders => "noheaders",
            GnuTable::NoHeading => "no-heading",
            GnuTable::NoHeadings => "no-headings",
            GnuTable::Noheading => "noheading",
            GnuTable::Noheadings => "noheadings",
            GnuTable::Pid => "pid",
            GnuTable::Ppid => "ppid",
            GnuTable::QuickPid => "quick-pid",
            GnuTable::Rows => "rows",
            GnuTable::Sid => "sid",
            GnuTable::Sort => "sort",
            GnuTable::Tty => "tty",
            GnuTable::Version => "version",
            GnuTable::Width => "width",
        }
    }
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
                                },
                            }
                        }
                        _ => ArgType::ArgFail,
                    },
                }
            }
            _ => ArgType::ArgFail,
        },
    }
}

pub fn arg_parse(args: Vec<String>) -> Result<LinkedList<Box<SelectionNode>>, String> {
    let mut selection_list = LinkedList::new();
    match parse_all_options(&args) {
        Ok(mut list) => {
            selection_list.append(&mut list);
        }
        Err(msg) => return Err(msg),
    };

    Ok(selection_list)
}

pub fn parse_all_options(args: &Vec<String>) -> Result<LinkedList<Box<SelectionNode>>, String> {
    let mut selection_list = LinkedList::new();
    for (ix, arg) in args.iter().enumerate() {
        log::trace!("argv[{}] is {}", ix, arg);
        if ix == 0 {
            log::trace!("skip this arg");
            continue;
        }
        match arg_type(&arg) {
            ArgType::ArgBsd => {
                log::trace!("{}: type ArgBsd", arg);
            }
            ArgType::ArgSysv => {
                log::trace!("{}: type ArgSysv", arg);
            }
            ArgType::ArgGnu => {
                log::trace!("{}: type ArgGnu", arg);
                match parse_gnu_option(arg) {
                    Ok(mut list) => {
                        selection_list.append(&mut list);
                        log::trace!("parse_all_options: {:?}", selection_list);
                    }
                    Err(msg) => return Err(msg),
                }
            }
            ArgType::ArgPgrp | ArgType::ArgSess | ArgType::ArgPid => {
                log::trace!("{}: type Arg{{Pgrp|Sess|Pid}}", arg);
            }
            ArgType::ArgFail => {
                log::trace!("{}: type ArgFail", arg);
                return Err(String::from("ArgFail"));
            }
        }
    }
    Ok(selection_list)
}

pub fn parse_gnu_option(arg: &String) -> Result<LinkedList<Box<SelectionNode>>, String> {
    log::trace!("parse_gnu_option()");

    let mut selection_list = LinkedList::new();
    let argpair = arg.split("=").collect::<Vec<&str>>(); // XXX don't support ":" separator for now.
    if argpair.len() != 2 {
        return Err(String::from("unknown gnu long option"));
    }
    let argname = match GnuTable::from_str(&argpair[0][2..]) {
        Some(a) => a,
        None => return Err(String::from("unknown gnu long option")),
    };
    let val = argpair[1];
    match argname {
        GnuTable::Pid => {
            // XXX now support only "--pid=xx" style
            log::trace!("--pid processing");
            let pid = match val.parse() {
                Ok(p) => p,
                Err(_) => return Err(String::from("cannot parse given PID")),
            };
            selection_list.push_front(Box::new(SelectionNode {
                n: 1,
                u: Box::new(SelUnion { val: pid }),
                typecode: SelectionListType::SelPid,
            }));
        }
        _ => unimplemented!(),
    };

    log::trace!("{:?}", selection_list);
    return Ok(selection_list);
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
        assert_eq!(
            super::arg_type(&String::from("+3")),
            super::ArgType::ArgSess
        );
        assert_eq!(
            super::arg_type(&String::from("-3")),
            super::ArgType::ArgPgrp
        );
        assert_eq!(
            super::arg_type(&String::from("-ax")),
            super::ArgType::ArgSysv
        );
        assert_eq!(super::arg_type(&String::from("-")), super::ArgType::ArgFail);
        assert_eq!(
            super::arg_type(&String::from("- ")),
            super::ArgType::ArgFail
        );
        assert_eq!(
            super::arg_type(&String::from("--ax")),
            super::ArgType::ArgGnu
        );
        assert_eq!(
            super::arg_type(&String::from("--AX")),
            super::ArgType::ArgGnu
        );
        assert_eq!(
            super::arg_type(&String::from("---")),
            super::ArgType::ArgFail
        );
        assert_eq!(
            super::arg_type(&String::from("--")),
            super::ArgType::ArgFail
        );
        assert_eq!(
            super::arg_type(&String::from("--3")),
            super::ArgType::ArgFail
        );
    }

    #[test]
    fn parse_gnu_option_pid_1() {
        let args = vec![String::from("me"), String::from("--pid=39")];
        let b0 = Box::new(super::SelectionNode {
            u: Box::new(super::SelUnion { val: 39 }),
            n: 1,
            typecode: super::SelectionListType::SelPid,
        });
        match super::arg_parse(args) {
            Ok(mut list) => {
                assert_eq!(list.pop_front().unwrap(), b0);
            }
            Err(msg) => panic!(msg),
        }
    }

    #[test]
    fn parse_gnu_option_pid_2() {
        let args = vec![
            String::from("me"),
            String::from("--pid=39"),
            String::from("--pid=123"),
        ];
        let b0 = Box::new(super::SelectionNode {
            u: Box::new(super::SelUnion { val: 39 }),
            n: 1,
            typecode: super::SelectionListType::SelPid,
        });
        let b1 = Box::new(super::SelectionNode {
            u: Box::new(super::SelUnion { val: 123 }),
            n: 1,
            typecode: super::SelectionListType::SelPid,
        });
        match super::arg_parse(args) {
            Ok(mut list) => {
                assert_eq!(list.pop_front().unwrap(), b0);
                assert_eq!(list.pop_front().unwrap(), b1);
            }
            Err(msg) => panic!(msg),
        }
    }
}
