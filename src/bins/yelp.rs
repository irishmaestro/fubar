pub static BIN_NAME: &'static str = "yelp";
pub static FR_DESC: &'static str = "This spawns a graphical window containing the file content somehow corrupted by word wrapping, it might not be suitable to read arbitrary files. The path must be absolute.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    yelp "man:$LFILE"
"#;
use crate::code::{Code, Tag};
pub static FR: Code<'static> = Code { 
	title: "FR_CODE",
	code: FR_CODE,
	tag: Tag::FR,
};
