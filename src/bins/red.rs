pub static BIN_NAME: &'static str = "red";
pub static BIN_DESC: &'static str = "Read and write files limited to the current directory";
pub static FW_CODE: &'static str = r#"
    
    red file_to_write
    a
    DATA
    .
    w
    q
"#;
pub static FR_CODE: &'static str = r#"
    
    red file_to_read
    ,p
    q
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo red file_to_write
    a
    DATA
    .
    w
    q
"#;
use crate::code::{Code, Tag};
pub static FW: Code<'static> = Code { 
	title: "FW_CODE",
	code: FW_CODE,
	tag: Tag::FW,
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
