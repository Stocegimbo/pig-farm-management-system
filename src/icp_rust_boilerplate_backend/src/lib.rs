#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct Pig {
    id: u64,
    name: String,
    breed: String,
    birth_date: u64,
    weight: f64,
    health_status: String,
    created_at: u64,
}

impl Pig {
    fn new(
        id: u64,
        name: String,
        breed: String,
        birth_date: u64,
        weight: f64,
        health_status: String,
    ) -> Self {
        Self {
            id,
            name,
            breed,
            birth_date,
            weight,
            health_status,
            created_at: time(),
        }
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct Feed {
    id: u64,
    pig_id: u64,
    feed_type: String,
    quantity: f64,
    date: u64,
    created_at: u64,
}

impl Feed {
    fn new(id: u64, pig_id: u64, feed_type: String, quantity: f64, date: u64) -> Self {
        Self {
            id,
            pig_id,
            feed_type,
            quantity,
            date,
            created_at: time(),
        }
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct HealthRecord {
    id: u64,
    pig_id: u64,
    description: String,
    date: u64,
    veterinarian: String,
    created_at: u64,
}

impl HealthRecord {
    fn new(id: u64, pig_id: u64, description: String, date: u64, veterinarian: String) -> Self {
        Self {
            id,
            pig_id,
            description,
            date,
            veterinarian,
            created_at: time(),
        }
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct Inventory {
    id: u64,
    item_name: String,
    quantity: u64,
    cost: f64,
    created_at: u64,
}

impl Inventory {
    fn new(id: u64, item_name: String, quantity: u64, cost: f64) -> Self {
        Self {
            id,
            item_name,
            quantity,
            cost,
            created_at: time(),
        }
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct Invoice {
    id: u64,
    customer_id: u64,
    amount: f64,
    date: u64,
    created_at: u64,
}

impl Invoice {
    fn new(id: u64, customer_id: u64, amount: f64, date: u64) -> Self {
        Self {
            id,
            customer_id,
            amount,
            date,
            created_at: time(),
        }
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct Notification {
    id: u64,
    customer_id: u64,
    message: String,
    date: u64,
    is_read: bool,
}

impl Notification {
    fn new(id: u64, customer_id: u64, message: String, date: u64) -> Self {
        Self {
            id,
            customer_id,
            message,
            date,
            is_read: false,
        }
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct BreedingRecord {
    id: u64,
    pig_id: u64,
    mate_id: u64,
    date: u64,
    offspring_count: u64,
    created_at: u64,
}

impl BreedingRecord {
    fn new(id: u64, pig_id: u64, mate_id: u64, date: u64, offspring_count: u64) -> Self {
        Self {
            id,
            pig_id,
            mate_id,
            date,
            offspring_count,
            created_at: time(),
        }
    }
}

impl Storable for Pig {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Pig {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Feed {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Feed {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for HealthRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for HealthRecord {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Inventory {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Inventory {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Invoice {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Invoice {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Notification {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Notification {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for BreedingRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for BreedingRecord {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static PIGS_STORAGE: RefCell<StableBTreeMap<u64, Pig, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static FEEDS_STORAGE: RefCell<StableBTreeMap<u64, Feed, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static HEALTH_RECORDS_STORAGE: RefCell<StableBTreeMap<u64, HealthRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static INVENTORY_STORAGE: RefCell<StableBTreeMap<u64, Inventory, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static INVOICES_STORAGE: RefCell<StableBTreeMap<u64, Invoice, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));

    static NOTIFICATIONS_STORAGE: RefCell<StableBTreeMap<u64, Notification, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));

    static BREEDING_RECORDS_STORAGE: RefCell<StableBTreeMap<u64, BreedingRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7)))
    ));
}

// Payloads Definitions

#[derive(candid::CandidType, Deserialize, Serialize)]
struct PigPayload {
    name: String,
    breed: String,
    birth_date: u64,
    weight: f64,
    health_status: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct FeedPayload {
    pig_id: u64,
    feed_type: String,
    quantity: f64,
    date: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct HealthRecordPayload {
    pig_id: u64,
    description: String,
    date: u64,
    veterinarian: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct InventoryPayload {
    item_name: String,
    quantity: u64,
    cost: f64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct InvoicePayload {
    customer_id: u64,
    amount: f64,
    date: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct NotificationPayload {
    customer_id: u64,
    message: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct BreedingRecordPayload {
    pig_id: u64,
    mate_id: u64,
    date: u64,
    offspring_count: u64,
}

// Functions to create and get entities

#[ic_cdk::update]
fn add_pig(payload: PigPayload) -> Result<Pig, String> {
    // Validate the payload
    if payload.name.is_empty() || payload.breed.is_empty() || payload.health_status.is_empty() {
        return Err("Name, breed and health status are required fields".to_string());
    }

    // Create a new pig
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });

    let pig = Pig::new(
        id,
        payload.name,
        payload.breed,
        payload.birth_date,
        payload.weight,
        payload.health_status,
    );

    PIGS_STORAGE.with(|storage| storage.borrow_mut().insert(pig.id, pig.clone()));

    Ok(pig)
}

// Function to update a pig
#[ic_cdk::update]
fn update_pig(id: u64, payload: PigPayload) -> Result<Pig, String> {
    // Validate the pig_id
    if !PIGS_STORAGE.with(|storage| storage.borrow().contains_key(&id)) {
        return Err("Pig not found".to_string());
    }

    // Validate the payload
    if payload.name.is_empty() || payload.breed.is_empty() || payload.health_status.is_empty() {
        return Err("Name, breed and health status are required fields".to_string());
    }

    // Update the pig
    let pig = Pig::new(
        id,
        payload.name,
        payload.breed,
        payload.birth_date,
        payload.weight,
        payload.health_status,
    );

    PIGS_STORAGE.with(|storage| storage.borrow_mut().insert(pig.id, pig.clone()));

    Ok(pig)
}

#[ic_cdk::query]
fn get_all_pigs() -> Result<Vec<Pig>, String> {
    PIGS_STORAGE.with(|storage| {
        let pigs: Vec<Pig> = storage
            .borrow()
            .iter()
            .map(|(_, pig)| pig.clone())
            .collect();
        if pigs.is_empty() {
            Err("No pigs found".to_string())
        } else {
            Ok(pigs)
        }
    })
}

#[ic_cdk::update]
fn create_feed(payload: FeedPayload) -> Result<Feed, String> {
    // Validate the payload
    if payload.feed_type.is_empty() || payload.quantity == 0.0 {
        return Err("Feed type and quantity are required fields".to_string());
    }

    // Validate the pig_id
    if !PIGS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.pig_id)) {
        return Err("Pig not found".to_string());
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });
    let feed = Feed::new(
        id,
        payload.pig_id,
        payload.feed_type,
        payload.quantity,
        payload.date,
    );
    FEEDS_STORAGE.with(|storage| storage.borrow_mut().insert(feed.id, feed.clone()));
    Ok(feed)
}

#[ic_cdk::query]
fn get_all_feeds() -> Result<Vec<Feed>, String> {
    FEEDS_STORAGE.with(|storage| {
        let feeds: Vec<Feed> = storage
            .borrow()
            .iter()
            .map(|(_, feed)| feed.clone())
            .collect();
        if feeds.is_empty() {
            Err("No feeds found".to_string())
        } else {
            Ok(feeds)
        }
    })
}

#[ic_cdk::update]
fn create_health_record(payload: HealthRecordPayload) -> Result<HealthRecord, String> {
    // Validate the payload
    if payload.description.is_empty() || payload.veterinarian.is_empty() {
        return Err("Description and veterinarian are required fields".to_string());
    }

    // Validate the pig_id
    if !PIGS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.pig_id)) {
        return Err("Pig not found".to_string());
    }

    // Create a new health record
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });
    let health_record = HealthRecord::new(
        id,
        payload.pig_id,
        payload.description,
        payload.date,
        payload.veterinarian,
    );
    HEALTH_RECORDS_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(health_record.id, health_record.clone())
    });
    Ok(health_record)
}

#[ic_cdk::query]
fn get_all_health_records() -> Result<Vec<HealthRecord>, String> {
    HEALTH_RECORDS_STORAGE.with(|storage| {
        let records: Vec<HealthRecord> = storage
            .borrow()
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No health records found".to_string())
        } else {
            Ok(records)
        }
    })
}

#[ic_cdk::update]
fn add_inventory_item(payload: InventoryPayload) -> Result<Inventory, String> {
    // Validate the payload
    if payload.item_name.is_empty() || payload.quantity == 0 || payload.cost == 0.0 {
        return Err("Item name, quantity and cost are required fields".to_string());
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });
    let inventory = Inventory::new(id, payload.item_name, payload.quantity, payload.cost);
    INVENTORY_STORAGE.with(|storage| storage.borrow_mut().insert(inventory.id, inventory.clone()));
    Ok(inventory)
}

#[ic_cdk::query]
fn get_all_inventory_items() -> Result<Vec<Inventory>, String> {
    INVENTORY_STORAGE.with(|storage| {
        let inventory: Vec<Inventory> = storage
            .borrow()
            .iter()
            .map(|(_, item)| item.clone())
            .collect();
        if inventory.is_empty() {
            Err("No inventory items found".to_string())
        } else {
            Ok(inventory)
        }
    })
}

#[ic_cdk::update]
fn create_invoice(payload: InvoicePayload) -> Result<Invoice, String> {
    // Validate the payload
    if payload.amount == 0.0 {
        return Err("Amount is a required field".to_string());
    }

    // Validate the customer_id
    if !PIGS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.customer_id)) {
        return Err("Customer not found".to_string());
    }

    // Create a new invoice
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });
    let invoice = Invoice::new(id, payload.customer_id, payload.amount, payload.date);
    INVOICES_STORAGE.with(|storage| storage.borrow_mut().insert(invoice.id, invoice.clone()));
    Ok(invoice)
}

// Function to get all invoices for a customer(customer_id)
#[ic_cdk::query]
fn get_all_invoices(customer_id: u64) -> Result<Vec<Invoice>, String> {
    INVOICES_STORAGE.with(|storage| {
        let invoices: Vec<Invoice> = storage
            .borrow()
            .iter()
            .filter(|(_, invoice)| invoice.customer_id == customer_id)
            .map(|(_, invoice)| invoice.clone())
            .collect();
        if invoices.is_empty() {
            Err("No invoices found".to_string())
        } else {
            Ok(invoices)
        }
    })
}

#[ic_cdk::update]
fn create_notification(payload: NotificationPayload) -> Result<Notification, String> {
    // Validate the payload
    if payload.message.is_empty() {
        return Err("Message is a required field".to_string());
    }

    // Validate the customer id
    if !PIGS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.customer_id)) {
        return Err("Customer not found".to_string());
    }

    // Create a new notification
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });
    let notification = Notification::new(id, payload.customer_id, payload.message, time());
    NOTIFICATIONS_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(notification.id, notification.clone())
    });
    Ok(notification)
}

// Function to get all notifications for a user
#[ic_cdk::query]
fn get_all_notifications(customer_id: u64) -> Result<Vec<Notification>, String> {
    NOTIFICATIONS_STORAGE.with(|storage| {
        let notifications: Vec<Notification> = storage
            .borrow()
            .iter()
            .filter(|(_, notification)| notification.customer_id == customer_id)
            .map(|(_, notification)| notification.clone())
            .collect();
        if notifications.is_empty() {
            Err("No notifications found".to_string())
        } else {
            Ok(notifications)
        }
    })
}

#[ic_cdk::update]
fn create_breeding_record(payload: BreedingRecordPayload) -> Result<BreedingRecord, String> {
    // Validate the payload
    if payload.offspring_count == 0 {
        return Err("Offspring count is a required field".to_string());
    }

    // Validate the pig_id and mate_id
    if !PIGS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.pig_id))
        || !PIGS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.mate_id))
    {
        return Err("Pig or mate not found".to_string());
    }

    // Validate to ensure that the pig and mate are not the same
    if payload.pig_id == payload.mate_id {
        return Err("Pig and mate cannot be the same".to_string());
    }

    // Create a new breeding record
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });
    let breeding_record = BreedingRecord::new(
        id,
        payload.pig_id,
        payload.mate_id,
        payload.date,
        payload.offspring_count,
    );
    BREEDING_RECORDS_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(breeding_record.id, breeding_record.clone())
    });
    Ok(breeding_record)
}

// Function to fetch breeding records for a specific pig(pig_id)
#[ic_cdk::query]
fn get_breeding_records(pig_id: u64) -> Result<Vec<BreedingRecord>, String> {
    BREEDING_RECORDS_STORAGE.with(|storage| {
        let records: Vec<BreedingRecord> = storage
            .borrow()
            .iter()
            .filter(|(_, record)| record.pig_id == pig_id)
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No breeding records found".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to fetch breeding records for a specific pig(mate_id)
#[ic_cdk::query]
fn get_mate_breeding_records(mate_id: u64) -> Result<Vec<BreedingRecord>, String> {
    BREEDING_RECORDS_STORAGE.with(|storage| {
        let records: Vec<BreedingRecord> = storage
            .borrow()
            .iter()
            .filter(|(_, record)| record.mate_id == mate_id)
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No breeding records found".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to get all breeding records
#[ic_cdk::query]
fn get_all_breeding_records() -> Result<Vec<BreedingRecord>, String> {
    BREEDING_RECORDS_STORAGE.with(|storage| {
        let records: Vec<BreedingRecord> = storage
            .borrow()
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No breeding records found".to_string())
        } else {
            Ok(records)
        }
    })
}

// Error types
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    UnAuthorized { msg: String },
}

// need this to generate candid
ic_cdk::export_candid!();
