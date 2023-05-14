pub static BIN_NAME: &'static str = "screen";
pub static SH_CODE: &'static str = r#"
    
    screen
"#;
pub static FW_DESC_1: &'static str = r#"This works on screen version 4.06.02. Data is appended to the file and `\n` is converted to `\r\n`."#;
pub static FW_CODE_1: &'static str = r#"
    
    LFILE=file_to_write
    screen -L -Logfile $LFILE echo DATA
"#;
pub static FW_DESC_2: &'static str = r#"This works on screen version 4.05.00. Data is appended to the file and `\n` is converted to `\r\n`."#;
pub static FW_CODE_2: &'static str = r#"
    
    LFILE=file_to_write
    screen -L $LFILE echo DATA
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo screen
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SH_CODE",
    code: SH_CODE,
    tag: Tag::SH,
};
pub static FW_1: Code<'static> = Code {
    title: "FW_CODE_1",
    code: FW_CODE_1,
    tag: Tag::FW,
};
pub static FW_2: Code<'static> = Code {
    title: "FW_CODE_2",
    code: FW_CODE_2,
    tag: Tag::FW,
};
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
