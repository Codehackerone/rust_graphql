// Import the error trait from std library
use std::error::Error;
// Import Mutex and Arc on the sync module from std library to provide thread-safety. 
//Mutual Exclusion between many threads sharing data
use std::sync::{Arc, Mutex};
// Import the csv crate
use csv;
//Import client, invoice and invoice item model to be stored in this DataContext
use crate::models::*;

// Define a DataContext struct that will hold the Vec of each model with a mutex. A mutex is used to make the Vec thread-safe.
pub struct DataContext {
    pub clients: Arc<Mutex<Vec<ClientModel>>>,
    pub invoices: Arc<Mutex<Vec<InvoiceModel>>>,
    pub invoice_items: Arc<Mutex<Vec<InvoiceItemsModel>>>,
}

impl DataContext {
    // Create a function called init() for DataContext that will return a result with an instance of the same DataContext
    pub fn init() -> Result<DataContext, Box<dyn Error>> {
        // Read data from csv files in Client.csv
        let clients = read_from_file("./data/Clients.csv")?;
        // Read data from csv files in Invoices.csv
        let invoices = read_from_file("./data/Invoices.csv")?;
        // Read data from csv files in InvoiceItems.csv
        let invoice_items = read_from_file("./data/InvoiceItems.csv")?;

        // Return Ok with new DataContext containing the reading results, wrapped inside Arc and Mutex(Rust's safe Multi-Threading Mechanism) .
        Ok(DataContext {
            clients: Arc::new(Mutex::new(clients)),
            invoices: Arc::new(Mutex::new(invoices)),
            invoice_items: Arc::new(Mutex::new(invoice_items)),
        })
    }
}

fn read_from_file<T>(path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: serde::de::DeserializeOwned,
{
    //Print the path of file on console(Note: This line is optional, just for log purposes)
    println!("{}", path);
    //Read the CSV file at given path using the csv crate and handle any potential errors by returning the error instance as Boxed dyonamic Error object
    let mut reader = csv::Reader::from_path(path)?;

    let mut results = Vec::new();

    //Iterate over the CSV rows, Deserialize each row into a record (of generic type), and store the resulting record in a vector that will be returned.
    for result in reader.deserialize() {
        let record: T = result?;

        results.push(record);
    }

    Ok(results)
}

// std::error::Error - This is a standard library module that defines the trait for errors that can be returned by Rust functions. It provides a common interface for working with errors, including error messages and backtraces.

// std::sync::{Arc, Mutex} - These are standard library modules that provide support for creating thread-safe data structures. Arc stands for "atomically reference count", which allows multiple threads to have shared ownership of an object. Mutex stands for "mutual exclusion", which allows threads to block each other from accessing a portion of code or data structure while one thread modifies it.

// csv - This is an external crate (not a part of the standard library) that provides CSV parsing and serialization functionality. It allows reading data from CSV files and deserializing them into Rust structs.

// serde::de::DeserializeOwned - This is a trait defined in the serde crate (an external dependency) that enables structs to be deserialized from data formats supported by serde.