use std::error::Error;

use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Element {
    atomic_number: u8,
    symbol: [u8; 1..=2],
    name: String,
    atomic_mass: f64,
    cpk_hex_color: [u8; 3],
    electron_configuration: &[u8],
    electronegativity: Option<f64>,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
