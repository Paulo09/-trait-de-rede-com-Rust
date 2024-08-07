pub struct Router {
    pub ip: String,
}

pub trait Network {
    fn ping(&self, host: &str) -> bool {
        println!("Pinging host: {}", host);
        true // Simulando sucesso do ping
    }

    fn traceroute(&self, host: &str) -> Vec<String> {
        println!("Tracing route to: {}", host);
        vec!["192.168.1.1".to_string(), "10.0.0.1".to_string()] // Simulando resultados do traceroute
    }

    fn nslookup(&self, host: &str) -> String {
        println!("Looking up DNS for: {}", host);
        "8.8.8.8".to_string() // Simulando resultado do nslookup
    }
}

impl Network for Router {
    // Implementações dos métodos já estão definidas na trait
}
