pub static BIN_NAME: &'static str = "script";
pub static SH_CODE: &'static str = r#"
    
    script -q /dev/null
"#;
pub static FW_DESC: &'static str = "The wrote content is corrupted by debug prints.";
pub static FW_CODE: &'static str = r#"
    
    script -q -c 'echo DATA' file_to_write
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo script -q /dev/null
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static FW: Code<'static> = Code { 
	title: "FW_CODE",
	code: FW_CODE,
	tag: Tag::FW,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
