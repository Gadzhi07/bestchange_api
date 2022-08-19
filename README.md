# Bestchange API

Library for convenient use of the bestchange api.

## Example

```rust,no_run
extern crate bestchange_api;

fn main() {
    bestchange_api::init();

    let cities = bestchange_api::City::load();
    let rates = bestchange_api::Rate::load();
    let exchangers = bestchange_api::Exchanger::load();
    let currencies = bestchange_api::Currency::load();

    let city_for_id = bestchange_api::City::get_by_id(&cities, 111_u16).unwrap();
    println!("{:?}", city_for_id);

    let currencies_for_id208 = bestchange_api::Currency::get_by_id(&currencies, 208_u16).unwrap();
    let currencies_for_id93 = bestchange_api::Currency::get_by_id(&currencies, 93_u16).unwrap();
    println!("{:?}\n{:?}", currencies_for_id208, currencies_for_id93);

    let rates_from_id = bestchange_api::Rate::get(&rates, 208_u16, 93_u16);
    
    for rate in rates_from_id {
        if rate.min_sum <= 20_f32 {
            println!("{:?}", rate);
            let exchange_for_id = bestchange_api::Exchanger::get_by_id(&exchangers, rate.exchange_id).unwrap();
            println!("{:?}", exchange_for_id);
        }
    }
}

```
