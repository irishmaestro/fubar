#[derive(Debug)]
pub struct Tag<'a> {
    pub name: &'a str,
    pub desc: &'a str,
}

pub static SHELL: Tag<'static> = Tag { name: "shell", desc: "It can be used to break out from restricted environments by spawning an interactive system shell." };
pub static COMMAND: Tag<'static> = Tag { name: "command", desc: "It can be used to break out from restricted environments by running noninteractive system commands." };
pub static REVERSE_SHELL: Tag<'static> = Tag {
    name: "reverse_shell",
    desc:
        "It can send back a reverse shell to a listening attacker to open a remote network access.",
};
pub static NONINTERACTIVE_REVERSE_SHELL: Tag<'static> = Tag { name: "noninteractive_reverse_shell", desc: "It can send back a non-interactive reverse shell to a listening attacker to open a remote network access." };
pub static BIND_SHELL: Tag<'static> = Tag {
    name: "bind_shell",
    desc: "It can bind a shell to a local port to allow remote network access",
};
pub static NONINTERACTIVE_BIND_SHELL: Tag<'static> = Tag {
    name: "noninteractive_bind_shell",
    desc: "It can bind a noninteractive shell to a local port to allow remote network access",
};
pub static FILE_UPLOAD: Tag<'static> = Tag {
    name: "file_upload",
    desc: "It can exfiltrate files on the network.",
};
pub static FILE_DOWNLOAD: Tag<'static> = Tag {
    name: "file_download",
    desc: "It can download remote files.",
};
pub static FILE_WRITE: Tag<'static> = Tag { name: "file_write", desc: "It writes data to files, it may be used to do privileged writes or write files outside a restricted file system." };
pub static FILE_READ: Tag<'static> = Tag { name: "file_read", desc: "It reads data from files, it may be used to do privileged reads or disclose files outside a restricted file system." };
pub static LIBRARY_LOAD: Tag<'static> = Tag {
    name: "library_load",
    desc: "It loads shared libraries that may be used to run code in the binary execution context.",
};
pub static SUID: Tag<'static> = Tag {
    name: "suid",
    desc: r#"If the binary has the SUID bit set, it does not drop the elevated privileges and may be abused to access the file system, escalate or maintain privileged access as a SUID backdoor. If it is used to
run `sh -p`, omit the `-p` argument on systems like Debian (<= Stretch) that allow the default `sh` shell to run with SUID privileges. This example creates a local SUID copy of the binary and runs it
to maintain elevated privileges. To interact with an existing SUID binary skip the first command and run the program using its original path."#,
};
pub static SUDO: Tag<'static> = Tag { name: "sudo", desc: "If the binary is allowed to run as superuser by `sudo`, it does not drop the elevated privileges and may be used to access the file system, escalate or maintain privileged access." };
pub static CAPABILITIES: Tag<'static> = Tag {
    name: "capabilities",
    desc: r#"If the binary has the Linux `CAP_SETUID` capability set or it is executed by another binary with the capability set, it can be used as a backdoor to maintain privileged access by 
manipulating its own process UID."#,
};
pub static LIMITED_SUID: Tag<'static> = Tag {
    name: "limited_suid",
    desc: r#"If the binary has the SUID bit set, it may be abused to access the file system, escalate or maintain access with elevated privileges working as a SUID backdoor. If it is used to run commands 
(e.g. via `system()`-like invocations) it only works on systems like Debian (<= Stretch) that allow the default `sh` shell to run with SUID privileges. This example creates a local SUID copy of 
the binary and runs it to maintain elevated privileges. To interact with an existing SUID binary skip the first command and run the program using its original path."#,
};
