use crate::wifi::{self, WifiNetwork};

pub struct App {
    pub networks: Vec<WifiNetwork>,
    pub selected_index: usize,
    pub connected_ssid: Option<String>,
    pub error: Option<String>,
    pub input_mode: bool,
    pub password_input: String,
    pub connecting_ssid: Option<String>,
    pub message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            networks: Vec::new(),
            selected_index: 0,
            connected_ssid: None,
            error: None,
            input_mode: false,
            password_input: String::new(),
            connecting_ssid: None,
            message: None,
        }
    }

    pub fn refresh(&mut self) -> Result<(), String> {
        self.networks = wifi::scan_wifi()?;
        self.connected_ssid = wifi::get_connected_ssid().ok();
        self.error = None;
        Ok(())
    }

    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.selected_index < self.networks.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn selected_network(&self) -> Option<&WifiNetwork> {
        self.networks.get(self.selected_index)
    }

    pub fn connect(&mut self, password: Option<&str>) -> Result<(), String> {
        let network = self.selected_network().ok_or("No network selected")?;
        let ssid = network.ssid.clone();
        self.connecting_ssid = Some(ssid.clone());
        self.message = Some(format!("Connecting to {}...", ssid));
        wifi::connect(&ssid, password)?;
        self.message = None;
        self.connecting_ssid = None;
        self.refresh()?;
        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), String> {
        wifi::disconnect()?;
        self.refresh()?;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.connected_ssid.is_some()
    }
}
