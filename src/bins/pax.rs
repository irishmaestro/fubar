pub static BIN_NAME: &'static str = "pax";
pub static FR_DESC: &'static str = "The output is a `tar` archive containing the read file as it is, hence this may not be suitable to read arbitrary binary files.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    pax -w "$LFILE"
"#;
use crate::code::{Code, Tag};
pub static FR: Code<'static> = Code { 
	title: "FR_CODE",
	code: FR_CODE,
	tag: Tag::FR,
};
