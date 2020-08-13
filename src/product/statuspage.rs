pub struct Client {
    page: String,
    base_api_endpoint: String,
    version: String,
    api_endpoint: String
}

impl Default for Client {
    fn default() -> Client {
        Client {
            page: String::from(""),
            base_api_endpoint: String::from("statuspage.io/api"),
            version: String::from("v2"),
            api_endpoint: String::from("")
        }
    }
}

impl Client {
    pub fn new(page: String) -> Self {
        let mut client = Client { page, ..Default::default() };
        client.api_endpoint = format!("https://{}.{}/{}",
                                      client.page,
                                      client.base_api_endpoint,
                                      client.version);
        client
    }

    pub fn get_summary_uri(&self) -> String {
        format!("{}/summary.json", self.api_endpoint)
    }

    pub fn get_status_uri(&self) -> String {
        format!("{}/status.json", self.api_endpoint)
    }

    pub fn get_components_uri(&self) -> String {
        format!("{}/components.json", self.api_endpoint)
    }

    pub fn get_unresolved_incidents_uri(&self) -> String {
        format!("{}/unresolved.json", self.api_endpoint)
    }

    pub fn get_recent_incidents_uri(&self) -> String {
        format!("{}/incidents.json", self.api_endpoint)
    }

    pub fn get_all_scheduled_maintenance_uri(&self) -> String {
        format!("{}/scheduled-maintenances.json", self.api_endpoint)
    }

    pub fn get_upcoming_scheduled_maintenance_uri(&self) -> String {
        format!("{}/scheduled-maintenances/upcoming.json", self.api_endpoint)
    }

    pub fn get_active_scheduled_maintenance_uri(&self) -> String {
        format!("{}/scheduled-maintenances/active.json", self.api_endpoint)
    }
}
