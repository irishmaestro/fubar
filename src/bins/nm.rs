pub static BIN_NAME: &'static str = "nm";
pub static BIN_DESC: &'static str = "The file content is treated as command line options and disclosed through error messages, so this is not suitable for reading arbitrary binary data.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    nm @$LFILE
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which nm) .

    LFILE=file_to_read
    ./nm @$LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo nm @$LFILE
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
