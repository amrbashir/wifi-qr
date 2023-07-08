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
