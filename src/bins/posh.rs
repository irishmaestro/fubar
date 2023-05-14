pub static BIN_NAME: &'static str = "posh";
pub static SH_CODE: &'static str = r#"
    
    posh
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo posh
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which posh) .
    ./posh
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
pub static LSUID: Code<'static> = Code { 
	title: "LSUID_CODE",
	code: LSUID_CODE,
	tag: Tag::LSUID,
};
