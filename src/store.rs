use std::{fs::File, io::{Read, Write}, path::PathBuf, time::Duration};
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::{self, time::timeout};

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Service {
    ip: String,
    port: u16,
    name: String,
}

pub struct ServiceList{
    services: Vec<Service>
}

fn get_dir() -> PathBuf {
    let homedir = dirs::home_dir().unwrap();
    let filepath = homedir.join("service_registry.json");
    filepath
}

impl ServiceList {
    pub fn load() -> Self {
        let filepath = get_dir();

        let mut file = match File::open(filepath) {
            Ok(f) => f,
            Err(_) => return ServiceList { services: Vec::new() }
        };

        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_err() {
            return ServiceList { services: Vec::new() };
        };

        let services: Vec<Service> = serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
        ServiceList { services: services }
    }

    fn save(&self) {
        let filepath = get_dir();
        
        let mut file = File::create(filepath).expect("Failed to create file");

        let json_string = serde_json::to_string_pretty(&self.services).expect("Failed to write file");

        file.write_all(json_string.as_bytes()).expect("Failed to save")
    }

    fn get_index(&self, name: &String) -> Result<usize, &'static str> {
        match self.services.iter().position(|d| &d.name == name) {
            Some(pos) => Ok(pos),
            None => Err("Not found")
        }
    }

    pub fn add(&mut self, ip: &String, port: &u16, name: &String) -> Result<(), &'static str> {
        if self.get_index(name).is_ok() {
            return Err("Directory already has been registered");
        };

        self.services.push(Service {ip: ip.clone(), port: port.clone(), name : name.clone()});
        self.save();
        Ok(())
    }

    pub fn delete(&mut self, name: &String) -> Result<(), &'static str> {
        let index = self.get_index(name)?;

        self.services.remove(index);
        self.save();
        Ok(())
    }

    pub fn list(&self) {
        for service in &self.services {
            println!("{}: {}:{}", service.name, service.ip, service.port)
        }
    }

    pub fn get_ip(&self, name: &String) -> Result<String, &'static str> {
        match self.get_index(name) {
            Ok(idx) => Ok(self.services[idx].ip.clone()),
            Err(e) => Err(e)
        }
    }

    pub async fn test(&self, name: &String) -> Result<&'static str, &'static str> {
        let idx = self.get_index(name)?;
        let service = self.services[idx].clone();

        match ping_ip(service.ip, service.port).await {
            true => {
                Ok("Online")
            },
            false => {
                Ok("Offline")
            }
        }
    }

    pub async fn test_all(&self) {
        let mut handles = vec![];

        for service in &self.services {
            let ip = service.ip.clone();
            let port = service.port;
            let name = service.name.clone();
            
            let handle = tokio::spawn(async move {
                let result = ping_ip(ip, port).await;
                println!("{}: {}", name, if result { "Online" } else { "Offline" })
            });

            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.await;
        }
    }

    pub fn update(&mut self, action: &String, name: &String, new_val: &String) -> Result<(), &'static str> {
        match action.to_lowercase().as_str() {
            "port" => {
                let new_port: u16 = new_val.parse().expect("Must be an integer");
                let idx = self.get_index(name)?;
                let service = &mut self.services[idx];

                service.port = new_port;
                self.save();
                Ok(())
            },
            "name" => {
                if self.get_index(new_val).is_ok() {
                    return Err("Service with that name already exists")
                }
                let idx = self.get_index(name)?;
                let service = &mut self.services[idx];

                service.name = new_val.clone();
                self.save();
                Ok(())
            },
            "ip" => {
                let idx = self.get_index(name)?;
                let service = &mut self.services[idx];

                service.ip = new_val.clone();
                self.save();
                Ok(())
            }
            _ => Err("Action must be Port/IP/Name")
        }
    }

}

async fn ping_ip(ip_addres: String, port: u16) -> bool {
    let target = format!("{ip_addres}:{port}");
    let connection_timeout = Duration::from_secs_f32(10.);

    match timeout(connection_timeout, tokio::net::TcpStream::connect(&target)).await {
        Ok(Ok(_stream)) => {
            true
        }
        _ => {
            false
        }
    }
}