
pub static BIN_NAME: &'static str = "fish";
pub static SH_CODE: &'static str = r#"
    
    fish
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which fish) .

    ./fish
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo fish
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
