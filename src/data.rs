// data.rs
// Loads Airbnb CSV data into Listing structs using serde and the csv crate.

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;
use serde::Deserialize;

// A single Airbnb listing
#[derive(Debug, Deserialize)]
pub struct Listing {
    pub neighbourhood_group: String, // Borough
    pub neighbourhood: String, //Neighborhood
    pub price: u32, //Price
    pub room_type: String, //Room type
}

// Loads all listings from CSV file into Vec<Listing>
pub fn load_listings(filepath: &str) -> Result<Vec<Listing>, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut csv_reader = Reader::from_reader(reader);

    let mut listings = Vec::new();

    for result in csv_reader.deserialize() {
        let record: Listing = result?;
        listings.push(record);
    }

    Ok(listings)
}
