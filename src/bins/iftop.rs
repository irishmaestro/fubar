pub static BIN_NAME: &'static str = "iftop";
pub static BIN_DESC: &'static str = "This requires `iftop` 0.17 and the privilege to capture on some device (specify with `-i` if needed) .";
pub static SH_CODE: &'static str = r#"
    
    iftop
    !/bin/sh
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo iftop
    !/bin/sh
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which iftop) .

    ./iftop
    !/bin/sh
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
