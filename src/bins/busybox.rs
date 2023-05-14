
pub static BIN_NAME: &'static str = "busybox";
pub static BIN_DESC: &'static str = "BusyBox may contain many UNIX utilities, run `busybox --list-full` to check what GTFOBins binaries are supported. Here some example.";
pub static SH_CODE: &'static str = r#"
    
    busybox sh
"#;
pub static FU_DESC: &'static str = "Serve files in the local folder running an HTTP server.";
pub static FU_CODE: &'static str = r#"
    
    LPORT=12345
    busybox httpd -f -p $LPORT -h .
"#;
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    busybox sh -c 'echo "DATA" > $LFILE'
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    ./busybox cat "$LFILE"
"#;
pub static SUID_DESC: &'static str = "It may drop the SUID privileges depending on the compilation flags and the runtime configuration.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which busybox) .

    ./busybox sh
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo busybox sh
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
