pub static BIN_NAME: &'static str = "vi";
pub static BIN_DESC: &'static str = "Modern Unix systems run `vim` binary when `vi` is called.";
pub static SH_CODE_1: &'static str = r#"
    
    vi -c ':!/bin/sh' /dev/null
"#;
pub static SH_CODE_2: &'static str = r#"
    
    vi
    :set shell=/bin/sh
    :shell
"#;
pub static FW_CODE: &'static str = r#"
    
    vi file_to_write
    iDATA
    ^[
    w
"#;
pub static FR_CODE: &'static str = r#"
    
    vi file_to_read
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo vi -c ':!/bin/sh' /dev/null
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
