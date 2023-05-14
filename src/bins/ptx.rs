pub static BIN_NAME: &'static str = "ptx";
pub static BIN_DESC: &'static str = "While the program is capable of reading the file, it outputs a “permuted index” of its content, thus altering it. Adjusting the options could yield more readable outputs.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    ptx -w 5000 "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which ptx) .

    LFILE=file_to_read
    ./ptx -w 5000 "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo ptx -w 5000 "$LFILE"
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
