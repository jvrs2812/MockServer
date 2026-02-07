use fake::{Fake, faker};
use rand::Rng;
use serde_json::{json, Map, Value};
use uuid::Uuid;

pub fn generate_fake_value(fake_type: &str, config: &Map<String, Value>) -> Value {
    match fake_type.to_lowercase().as_str() {
        "uuid" => Value::String(Uuid::new_v4().to_string()),
        
        "name" | "fullname" | "name.fullname" => {
            let name: String = faker::name::en::Name().fake();
            Value::String(name)
        }
        
        "firstname" | "name.firstname" => {
            let name: String = faker::name::en::FirstName().fake();
            Value::String(name)
        }
        
        "lastname" | "name.lastname" => {
            let name: String = faker::name::en::LastName().fake();
            Value::String(name)
        }
        
        "email" | "internet.email" => {
            let email: String = faker::internet::en::SafeEmail().fake();
            Value::String(email)
        }
        
        "username" | "internet.username" => {
            let username: String = faker::internet::en::Username().fake();
            Value::String(username)
        }
        
        "phone" | "phonenumber" => {
            let phone: String = faker::phone_number::en::PhoneNumber().fake();
            Value::String(phone)
        }
        
        "address" | "address.full" => {
            let street: String = faker::address::en::StreetName().fake();
            let city: String = faker::address::en::CityName().fake();
            let state: String = faker::address::en::StateName().fake();
            Value::String(format!("{}, {}, {}", street, city, state))
        }
        
        "city" | "address.city" => {
            let city: String = faker::address::en::CityName().fake();
            Value::String(city)
        }
        
        "country" | "address.country" => {
            let country: String = faker::address::en::CountryName().fake();
            Value::String(country)
        }
        
        "zipcode" | "address.zipcode" => {
            let zip: String = faker::address::en::ZipCode().fake();
            Value::String(zip)
        }
        
        "number" | "int" | "integer" => {
            let min = config.get("min").and_then(|v| v.as_i64()).unwrap_or(0);
            let max = config.get("max").and_then(|v| v.as_i64()).unwrap_or(100);
            let mut rng = rand::thread_rng();
            Value::Number(rng.gen_range(min..=max).into())
        }
        
        "float" | "decimal" => {
            let min = config.get("min").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let max = config.get("max").and_then(|v| v.as_f64()).unwrap_or(100.0);
            let mut rng = rand::thread_rng();
            let val: f64 = rng.gen_range(min..=max);
            json!(format!("{:.2}", val).parse::<f64>().unwrap_or(val))
        }
        
        "bool" | "boolean" => {
            let mut rng = rand::thread_rng();
            Value::Bool(rng.gen_bool(0.5))
        }
        
        "datetime" | "date" | "timestamp" => {
            let now = chrono::Utc::now();
            Value::String(now.to_rfc3339())
        }
        
        "word" | "lorem.word" => {
            let word: String = faker::lorem::en::Word().fake();
            Value::String(word)
        }
        
        "sentence" | "lorem.sentence" => {
            let sentence: String = faker::lorem::en::Sentence(3..8).fake();
            Value::String(sentence)
        }
        
        "paragraph" | "lorem.paragraph" => {
            let para: String = faker::lorem::en::Paragraph(2..5).fake();
            Value::String(para)
        }
        
        "company" | "company.name" => {
            let company: String = faker::company::en::CompanyName().fake();
            Value::String(company)
        }
        
        "jobtitle" | "job.title" => {
            let job: String = faker::job::en::Title().fake();
            Value::String(job)
        }
        
        "url" | "internet.url" => {
            let domain: String = faker::internet::en::DomainSuffix().fake();
            let word: String = faker::lorem::en::Word().fake();
            Value::String(format!("https://{}.{}", word, domain))
        }
        
        "ipv4" | "ip" => {
            let ip: std::net::Ipv4Addr = faker::internet::en::IPv4().fake();
            Value::String(ip.to_string())
        }
        
        "color" | "color.hex" => {
            let mut rng = rand::thread_rng();
            let r: u8 = rng.gen();
            let g: u8 = rng.gen();
            let b: u8 = rng.gen();
            Value::String(format!("#{:02x}{:02x}{:02x}", r, g, b))
        }
        
        _ => {
            // Default to a random string
            let word: String = faker::lorem::en::Word().fake();
            Value::String(word)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Map;

    #[test]
    fn test_uuid_generation() {
        let config = Map::new();
        let result = generate_fake_value("uuid", &config);
        assert!(result.is_string());
        let uuid_str = result.as_str().unwrap();
        assert!(Uuid::parse_str(uuid_str).is_ok());
    }

    #[test]
    fn test_number_generation_with_range() {
        let mut config = Map::new();
        config.insert("min".to_string(), json!(10));
        config.insert("max".to_string(), json!(20));
        
        let result = generate_fake_value("number", &config);
        assert!(result.is_number());
        let num = result.as_i64().unwrap();
        assert!(num >= 10 && num <= 20);
    }
}
