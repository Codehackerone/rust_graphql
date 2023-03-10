// Importing the NaiveDate struct from the chrono library and the Deserialize trait from serde.
use chrono::NaiveDate;
use serde::Deserialize;

// Defining a ClientModel struct that will hold information about clients.
#[derive(Clone, Deserialize)]
// Using a serde attribute to ensure that fields are named in PascalCase when serialized into JSON or Deserialized from JSON.
#[serde(rename_all = "PascalCase")]
pub struct ClientModel {
    pub client_id: String,
    pub company_name: String,
    pub contact_name: String,
    pub contact_title: String,
    pub phone: String,
    pub email: String,
}

// Defining an InvoiceModel struct that will hold information about invoices.
#[derive(Clone, Deserialize)]
// Using a serde attribute to ensure that fields are named in PascalCase when serialized into JSON or Deserialized from JSON.
#[serde(rename_all = "PascalCase")]
pub struct InvoiceModel {
    pub invoice_id: i32,
    pub invoice_number: String,
    pub client_id: String,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
}

// Defining an InvoiceItemsModel struct that will hold information about items belonging to invoices.
#[derive(Clone, Deserialize)]
// Using a serde attribute to ensure that fields are named in PascalCase when serialized into JSON or Deserialized from JSON.
#[serde(rename_all = "PascalCase")]
pub struct InvoiceItemsModel {
    pub item_id: i32,
    pub invoice_id: i32,
    pub product_id: i32,
    pub description: String,
    pub price: f64,
}

// The code defines three different structs: ClientModel, InvoiceModel, and InvoiceItemsModel. Each of these structs has multiple fields that define the data they are holding.

// The #[derive(Deserialize)] attribute is used for all three structs to allow easy deserialization from JSON strings. Additionally, the #[serde(rename_all = "PascalCase")] attribute is used for each struct to specify that field names should be PascalCased when serialized or deserialized.

// Lastly, the chrono::NaiveDate type is imported and used for the invoice_date and due_date fields in InvoiceModel. This allows for easy manipulation of dates at the naivedate level.