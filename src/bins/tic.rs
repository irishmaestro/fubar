pub static BIN_NAME: &'static str = "tic";
pub static BIN_DESC: &'static str = "The tic command translates a terminfo file from source format into compiled format. It will attempt to translate an arbitrary file and output the contents of the file on failure, so this may not be suitable to read arbitrary binary data.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    tic -C "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which tic) .

    LFILE=file_to_read
    ./tic -C "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo tic -C "$LFILE"
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
