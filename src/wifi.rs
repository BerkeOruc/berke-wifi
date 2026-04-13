use std::process::Command;

#[derive(Debug, Clone)]
pub struct WifiNetwork {
    pub ssid: String,
    pub bssid: String,
    pub signal: i32,
    pub freq: u32,
    pub security: String,
    pub connected: bool,
}

impl WifiNetwork {
    pub fn signal_bars(&self) -> String {
        let bars = match self.signal {
            0..=20 => 1,
            21..=40 => 2,
            41..=60 => 3,
            61..=80 => 4,
            _ => 5,
        };
        let filled = "=".repeat(bars);
        let empty = " ".repeat(5 - bars);
        format!("[{}{}]", filled, empty)
    }
}

pub fn scan_wifi() -> Result<Vec<WifiNetwork>, String> {
    let output = Command::new("nmcli")
        .args([
            "-t",
            "-f",
            "SSID,BSSID,SIGNAL,FREQ,MODE,SECURITY",
            "device",
            "wifi",
            "list",
        ])
        .output()
        .map_err(|e| format!("Failed to execute nmcli: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut networks = Vec::new();
    let connected_ssid = get_connected_ssid().ok();

    for line in stdout.lines() {
        let fields: Vec<&str> = line.split(':').collect();
        if fields.len() >= 5 {
            let ssid = fields[0].to_string();
            if ssid.is_empty() {
                continue;
            }

            let signal = fields[2].parse().unwrap_or(0);
            let freq = fields[3].trim_end_matches(" MHz").parse().unwrap_or(0);
            let security = fields.get(5).unwrap_or(&"").to_string();

            let connected = connected_ssid.as_ref().map_or(false, |s| s == &ssid);

            networks.push(WifiNetwork {
                bssid: fields[1].to_string(),
                ssid,
                signal,
                freq,
                security,
                connected,
            });
        }
    }

    networks.sort_by(|a, b| b.signal.cmp(&a.signal));

    Ok(networks)
}

pub fn get_connected_ssid() -> Result<String, String> {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "ACTIVE,SSID", "connection", "show", "--active"])
        .output()
        .map_err(|e| format!("Failed to execute nmcli: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let fields: Vec<&str> = line.split(':').collect();
        if fields.len() >= 2 && fields[0] == "yes" {
            return Ok(fields[1].to_string());
        }
    }

    Err("No active connection".to_string())
}

pub fn connect(ssid: &str, password: Option<&str>) -> Result<(), String> {
    let mut cmd = Command::new("nmcli");
    cmd.args(["device", "wifi", "connect", ssid]);

    if let Some(pwd) = password {
        cmd.arg("password").arg(pwd);
    }

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to execute nmcli: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(())
}

pub fn disconnect() -> Result<(), String> {
    let output = Command::new("nmcli")
        .args(["device", "disconnect", "wlan0"])
        .output()
        .map_err(|e| format!("Failed to execute nmcli: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(())
}

pub fn is_wifi_enabled() -> bool {
    let output = Command::new("nmcli").args(["radio", "wifi"]).output();

    output.map_or(false, |o| {
        String::from_utf8_lossy(&o.stdout).trim() == "enabled"
    })
}
