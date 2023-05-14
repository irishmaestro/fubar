pub static BIN_NAME: &'static str = "nroff";
pub static SH_CODE: &'static str = r#"
    
    TF=$(mktemp -d)
    echo '#!/bin/sh' > $TF/groff
    echo '/bin/sh' >> $TF/groff
    chmod +x $TF/groff
    GROFF_BIN_PATH=$TF nroff
"#;
pub static FR_DESC: &'static str = "The file is typeset and some warning messages may appear.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    nroff $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp -d)
    echo '#!/bin/sh' > $TF/groff
    echo '/bin/sh' >> $TF/groff
    chmod +x $TF/groff
    sudo GROFF_BIN_PATH=$TF nroff
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
