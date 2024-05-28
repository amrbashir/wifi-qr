use freya::prelude::*;
use qrcode_generator::QrCodeEcc;
use std::{
    fmt::Display,
    os::windows::process::CommandExt,
    process::Command,
    str::{FromStr, ParseBoolError},
};

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::upper_case_acronyms)]
pub enum ConnectionType {
    #[default]
    None,
    WEP,
    WPA,
    WPA2EAP,
}

impl FromStr for ConnectionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Open" => Ok(ConnectionType::None),
            "WEB" => Ok(ConnectionType::WEP),
            "WPA" => Ok(ConnectionType::WPA),
            "WPA2-EAP" => Ok(ConnectionType::WPA2EAP),
            _ => Err("Unknown Connection Type".to_string()),
        }
    }
}

impl std::fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConnectionType::None => "Open",
                ConnectionType::WEP => "WEB",
                ConnectionType::WPA => "WPA",
                ConnectionType::WPA2EAP => "WPA2-EAP",
            }
        )
    }
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WiFi {
    pub name: String,
    pub password: String,
    pub conn_type: ConnectionType,
    pub hidden: bool,
}

impl Display for WiFi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.name, self.password, self.conn_type, self.hidden
        )
    }
}

impl FromStr for WiFi {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',');
        let name = s.next().unwrap().to_string();
        let password = s.next().unwrap().to_string();
        let conn_type = ConnectionType::from_str(s.next().unwrap())?;
        let hidden = s
            .next()
            .unwrap()
            .parse()
            .map_err(|e: ParseBoolError| e.to_string())?;
        Ok(Self {
            name,
            password,
            conn_type,
            hidden,
        })
    }
}

impl WiFi {
    pub fn all() -> Vec<WiFi> {
        let profiles: Vec<String> = Command::new("netsh")
            .args(["wlan", "show", "profiles"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .ok()
            .map(|o| {
                let output = String::from_utf8_lossy(&o.stdout);
                output
                    .lines()
                    .filter(|l| l.contains("All User Profile"))
                    .map(|l| l.split_once(':').unwrap().1.trim().to_string())
                    .collect()
            })
            .unwrap_or_default();

        profiles
            .iter()
            .filter_map(|p| {
                Command::new("netsh")
                    .args(["wlan", "show", "profile", p.as_str(), "key=clear"])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output()
                    .ok()
                    .map(|o| {
                        let output = String::from_utf8_lossy(&o.stdout);
                        let mut wifi = WiFi::default();

                        for line in output.lines() {
                            let (mut key, mut value) = line.split_once(':').unwrap_or_default();
                            key = key.trim();
                            value = value.trim();

                            if key == "SSID name" {
                                wifi.name = value[1..value.len() - 1].to_string();
                            }

                            if key == "Key Content" {
                                wifi.password = value.to_string();
                            }

                            if key == "Network broadcast" {
                                wifi.hidden = value.contains("Connect even");
                            }

                            if key == "Authentication" {
                                if value.contains("WPA") && value.contains("Personal") {
                                    wifi.conn_type = ConnectionType::WPA;
                                } else if value.contains("WPA") && !value.contains("Personal") {
                                    wifi.conn_type = ConnectionType::WPA2EAP;
                                } else if value.contains("WEB") {
                                    wifi.conn_type = ConnectionType::WEP;
                                } else {
                                    wifi.conn_type = ConnectionType::None;
                                }
                            }
                        }

                        wifi
                    })
            })
            .collect()
    }

    fn qr_str(&self) -> String {
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
