pub mod trader {

    use reqwest::Client;
    use std::fs;
    use std::str::from_utf8;

    pub struct Trader {
        endpoints: Vec<String>,
        client: Client,
    }

    impl Trader {
        pub fn new(endpoint_file_path: &str) -> Result<Self, Box<dyn std::error::Error + 'static>> {
            let data = fs::read(&endpoint_file_path)?;
            let endpoint_str = match from_utf8(&data) { 
                Ok(s) => s,
                Err(e) => panic!("File {} is corrupted. Failed with error: {}", endpoint_file_path, e),
            };
            let endpoints_slc: Vec<&str> = endpoint_str.split("\n").collect();
            let endpoints = endpoints_slc.iter().map(|slc| {
                    slc.to_string()
                }).collect();
            
            let client = Client::new();
            Ok(Trader {
                endpoints,
                client,
            })
        }
    }
}







