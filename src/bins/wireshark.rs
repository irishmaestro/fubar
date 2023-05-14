pub static BIN_NAME: &'static str = "wireshark";
pub static SUDO_DESC: &'static str = "This technique can be used to write arbitrary files, i.e., the dump of one UDP packet.

After starting Wireshark, and waiting for the capture to begin, deliver the UDP packet, e.g., with nc (see below). The capture then stops and the packet dump can be saved:

    1. select the only received packet;

    2. right-click on “Data” from the “Packet Details” pane, and select “Export Packet Bytes…”;

    3. choose where to save the packet dump.
";
pub static SUDO_CODE: &'static str = r#"
    
    PORT=4444
    sudo wireshark -c 1 -i lo -k -f "udp port $PORT" &
    echo 'DATA' | nc -u 127.127.127.127 "$PORT"
"#;
use crate::code::{Code, Tag};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
