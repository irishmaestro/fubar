pub static BIN_NAME: &'static str = "tmux";
pub static SH_CODE: &'static str = r#"
    
    tmux
"#;
pub static FR_DESC: &'static str = "The file is read and parsed as a `tmux` configuration file, part of the first invalid line is returned in an error message.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    tmux -f $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo tmux
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
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
