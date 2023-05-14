
pub static BIN_NAME: &'static str = "dialog";
pub static BIN_DESC: &'static str = "The file is shown in an interactive TUI dialog, thus it is not suitable for binary/too big data.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    dialog --textbox "$LFILE" 0 0
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which dialog) .

    LFILE=file_to_read
    ./dialog --textbox "$LFILE" 0 0
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo dialog --textbox "$LFILE" 0 0
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
