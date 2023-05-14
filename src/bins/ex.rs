pub static BIN_NAME: &'static str = "ex";
pub static SH_CODE: &'static str = r#"
    
    ex
    !/bin/sh
"#;
pub static FW_CODE: &'static str = r#"
    
    ex file_to_write
    a
    DATA
    .
    w
    q
"#;
pub static FR_CODE: &'static str = r#"
    
    ex file_to_read
    ,p
    q
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo ex
    !/bin/sh
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SH_CODE",
    code: SH_CODE,
    tag: Tag::SH,
};
pub static FW: Code<'static> = Code {
    title: "FW_CODE",
    code: FW_CODE,
    tag: Tag::FW,
};
pub static FR: Code<'static> = Code {
    title: "FR_CODE",
    code: FR_CODE,
    tag: Tag::FR,
};
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
