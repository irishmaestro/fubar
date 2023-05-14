pub static BIN_NAME: &'static str = "vipw";
pub static BIN_DESC: &'static str = "This command allows to edit some designated files (`/etc/passwd`, `/etc/group`, `/etc/shadow` and `/etc/gshadow`) safely by spawning the default editor (falling back to `vim`, other functions may apply). Despite requiring superuser privileges to run, the editor is executed as the unprivileged user when run as SUID or with `sudo`.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which vipw) .

    ./vipw
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo vipw
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
