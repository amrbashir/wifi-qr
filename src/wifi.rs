use qrcode_generator::QrCodeEcc;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum ConnectionType {
    #[default]
    None,
    WEP,
    WPA,
    WPA2EAP,
}

impl std::fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConnectionType::None => "nopass",
                ConnectionType::WEP => "WEB",
                ConnectionType::WPA => "WPA",
                ConnectionType::WPA2EAP => "WPA2-EAP,",
            }
        )
    }
}

#[derive(Default, Clone)]
pub struct WiFi {
    pub name: String,
    pub password: String,
    pub conn_type: ConnectionType,
    pub hidden: bool,
}

impl WiFi {
    pub fn qr_str(&self) -> String {
        let mut qr = format!("WIFI:T:{};S:{};", self.conn_type, self.name);
        if !self.password.is_empty() {
            qr.push_str(&format!("P:{};", self.password))
        }
        qr.push_str(&format!("{};", if self.hidden { "H:true" } else { "" }));
        qr
    }

    pub fn qr(&self) -> Vec<u8> {
        qrcode_generator::to_png_to_vec(self.qr_str(), QrCodeEcc::Low, 1024).unwrap()
    }
}
