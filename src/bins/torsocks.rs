pub static BIN_NAME: &'static str = "torsocks";
pub static SH_CODE: &'static str = r#"
    
    torsocks /bin/sh
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo torsocks /bin/sh
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
