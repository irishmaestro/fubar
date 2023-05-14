
pub static BIN_NAME: &'static str = "check_cups";
pub static BIN_DESC: &'static str = "This is the `check` cups Nagios plugin, available in `/usr/lib/nagios/plugins/`. The read file content is limited to the first line.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    check_cups --extra-opts=@$LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo check_cups --extra-opts=@$LFILE
"#;
use crate::code::{Code, Tag};
pub static FR: Code<'static> = Code { 
	title: "FR_CODE",
	code: FR_CODE,
	tag: Tag::FR,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
