pub static BIN_NAME: &'static str = "msfconsole";
pub static BIN_DESC: &'static str = "This allows to spawn a `ruby` interpreter.";
pub static SH_CODE: &'static str = r#"
    
    msfconsole
    msf6 > irb
    >> system("/bin/sh")
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo msfconsole
    msf6 > irb
    >> system("/bin/sh")
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SH_CODE",
    code: SH_CODE,
    tag: Tag::SH,
};
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
