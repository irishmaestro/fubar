pub static BIN_NAME: &'static str = "man";
pub static BIN_DESC: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.";
pub static SH_CODE_1: &'static str = r#"
    
    man man
    !/bin/sh
"#;
pub static SH_DESC_2: &'static str =
    "This only works for GNU `man` and requires GNU `troff` (`groff` to be installed).";
pub static SH_CODE_2: &'static str = r#"
    
    man '-H/bin/sh #' man
"#;
pub static FR_CODE: &'static str = r#"
    
    man file_to_read
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo man man
    !/bin/sh
"#;
use crate::code::{Code, Tag};
pub static SH_1: Code<'static> = Code {
    title: "SH_CODE_1",
    code: SH_CODE_1,
    tag: Tag::SH,
};
pub static SH_2: Code<'static> = Code {
    title: "SH_CODE_2",
    code: SH_CODE_2,
    tag: Tag::SH,
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
