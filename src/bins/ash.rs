pub static BIN_NAME: &'static str = "ash";
pub static SH_CODE: &'static str = r#"
    
    ash
"#;
pub static FW_CODE: &'static str = r#"

    export LFILE=file_to_write
    ash -c 'echo DATA > $LFILE'
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which ash) .

    ./ash
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo ash
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
pub static SUID: Code<'static> = Code {
    title: "SUID_CODE",
    code: SUID_CODE,
    tag: Tag::SUID,
};
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
