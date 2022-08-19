extern crate reqwest;
extern crate encoding_rs;

use std::fs::File;
use std::io::copy;
use std::io::Read;
use std::path::Path;
use encoding_rs::WINDOWS_1251;

fn download_file(url: &str) {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Error creating client");
    let mut response = client.get(url).send().expect("Error getting file");
    let mut dest = File::create("info.zip").expect("Error creating file");
    copy(&mut response, &mut dest).expect("Error copying response");
}

fn unzip(zippath: &str, name: &str) -> String {
    let fname = Path::new(zippath);
    let file = File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    let mut file = archive.by_name(name).unwrap();
    let mut file_bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_bytes).unwrap();

    WINDOWS_1251.decode(&file_bytes).0.into_owned()
}


/// Downloading an archive to work with the API.
/// 
/// # Example
/// 
/// ```
/// bestchange_api::init();
/// ``` 
pub fn init() {
    download_file("http://api.bestchange.com/info.zip");
}

#[derive(Debug)]
pub struct Rate {
    pub give_id: u16,
    pub get_id: u16,
    pub exchange_id: u16,
    pub rate: f32,
    pub reserve: f32,
    pub reviews: f32,
    pub min_sum: f32,
    pub max_sum: f32,
    pub city_id: u16,
}

/// Working with Rates (bm_rates.dat)
/// 
/// # Example
/// 
/// ```
/// bestchange_api::init();
/// let rates = bestchange_api::Rate::load(); 
/// let rates_from_id = bestchange_api::Rate::get(&rates, 208_u16, 93_u16);
/// println!("{:?}", rates_from_id);
/// ```

impl Rate {
    pub fn load() -> Vec<Rate> {
        let mut rates: Vec<Rate> = Vec::new();
        let str_rates = unzip("info.zip", "bm_rates.dat");

        for rate in str_rates.lines() {  
            let values: Vec<&str> = rate.split(";").collect();
            let (give_id, get_id, exchange_id, reserve, reviews, min_sum, max_sum, city_id) = (
                values[0].parse().unwrap(), 
                values[1].parse().unwrap(), 
                values[2].parse().unwrap(), 
                values[5].parse().unwrap(), 
                values[6].parse().unwrap(), 
                values[8].parse().unwrap(),
                values[9].parse().unwrap(), 
                values[10].parse().unwrap(),
            );
            let (rate_give, rate_get): (f32, f32) = (values[3].parse().unwrap(), values[4].parse().unwrap());
            let rate = rate_give / rate_get;
            rates.push(Rate {give_id, get_id, exchange_id, rate, reserve, reviews, min_sum, max_sum, city_id});
        }
        
        return rates;
    }

    pub fn get(rates: &Vec<Rate>, give_id: u16, get_id: u16) -> Vec<&Rate> {
        let mut rates_by_id: Vec<&Rate> = Vec::new();

        for rate in rates {
            if rate.give_id == give_id && rate.get_id == get_id {
                rates_by_id.push(rate);
            }
        }
        rates_by_id.sort_by(|a, b| a.rate.partial_cmp(&b.rate).unwrap());
        return rates_by_id; 
    }
}


#[derive(Debug)]
pub struct City {
    pub id: u16,
    pub name: String,
}

/// Working with Cities (bm_cities.dat)
/// 
/// # Example
/// 
/// ```
/// bestchange_api::init();
/// let cities = bestchange_api::City::load(); 
/// let cities_from_id = bestchange_api::City::get_by_id(&cities, 1_u16).unwrap();
/// println!("{:?}", cities_from_id);
/// ```
impl City {
    pub fn load() -> Vec<City> {
        let mut cities: Vec<City> = Vec::new();
        let str_cities = unzip("info.zip", "bm_cities.dat");

        for city in str_cities.lines() {  
            let values: Vec<&str> = city.split(";").collect();
            cities.push(City {id: values[0].parse().unwrap(), name: values[1].to_string()});
        }
        
        return cities;
    }

    pub fn get_by_id(cities: &Vec<City>, id: u16) -> Result<&City, String> {
        for city in cities {
            if city.id == id { 
                return Ok(city); 
            }
        }
        return Err(format!("The city with id {} was not found!", id));
    }

}

#[derive(Debug)]
pub struct Exchanger {
    pub id: u16,
    pub name: String,
}

/// Working with Exchangers (bm_exch.dat)
/// 
/// # Example
/// 
/// ```
/// bestchange_api::init();
/// let exchangers = bestchange_api::Exchanger::load();
/// let exchange_for_id = bestchange_api::Exchanger::get_by_id(&exchangers, 1_u16).unwrap();
/// println!("{:?}", exchange_for_id);
/// ```


impl Exchanger {
    pub fn load() -> Vec<Exchanger> {
        let mut exchangers: Vec<Exchanger> = Vec::new();
        let str_exchangers = unzip("info.zip", "bm_exch.dat");

        for exchanger in str_exchangers.lines() {  
            let values: Vec<&str> = exchanger.split(";").collect();
            exchangers.push(Exchanger {id: values[0].parse().unwrap(), name: values[1].to_string()});
        }
        
        return exchangers;
    }

    pub fn get_by_id(exchangers: &Vec<Exchanger>, id: u16) -> Result<&Exchanger, String> {
        for exchanger in exchangers {
            if exchanger.id == id {
                return Ok(exchanger);
            }
        }
        return Err(format!("The exchanger with id {} was not found!", id));
    }

}

#[derive(Debug)]
pub struct Currency {
    pub id: u16,
    pub pos_id: u16,
    pub name: String,
}

/// Working with Currencies (bm_cy.dat)
/// 
/// # Example
/// 
/// ```
/// bestchange_api::init();
/// let currencies = bestchange_api::Currency::load();
/// let currencies_for_id208 = bestchange_api::Currency::get_by_id(&currencies, 93_u16).unwrap();
/// println!("{:?}", currencies_for_id208);
/// ```

impl Currency{
    pub fn load() -> Vec<Currency> {
        let mut currencies: Vec<Currency> = Vec::new();
        let str_currencies = unzip("info.zip", "bm_cy.dat");

        for currency in str_currencies.lines() {  
            let values: Vec<&str> = currency.split(";").collect();
            currencies.push(Currency {id: values[0].parse().unwrap(), pos_id: values[1].parse().unwrap(), name: values[2].to_string()});
        }
        
        return currencies;
    }

    pub fn get_by_id(currencies: &Vec<Currency>, id: u16) -> Result<&Currency, String> {
        for currency in currencies {
            if currency.id == id {
                return Ok(currency);
            }
        }
        return Err(format!("The currency with id {} was not found!", id));
    }

}
