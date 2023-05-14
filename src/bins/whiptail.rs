pub static BIN_NAME: &'static str = "whiptail";
pub static BIN_DESC: &'static str = "The file is shown in an interactive TUI dialog made for displaying text, arrows can be used to scroll long content.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    whiptail --textbox --scrolltext "$LFILE" 0 0
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which whiptail) .

    LFILE=file_to_read
    ./whiptail --textbox --scrolltext "$LFILE" 0 0
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo whiptail --textbox --scrolltext "$LFILE" 0 0
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
