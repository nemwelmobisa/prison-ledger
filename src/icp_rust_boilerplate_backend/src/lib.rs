#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// EmergencyContact struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EmergencyContact {
    name: String,
    phone_number: String,
    relationship: String,
}

// Prison struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Prison {
    id: u64,
    name: String,
    location: String,
    capacity: u64,
    inmates: Vec<u64>, // List of inmate IDs housed in the prison
}

// Inmate struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Inmate {
    id: u64,
    name: String,
    age: u64,
    gender: String,
    sentence_details: String, // Sentence details, including start, end, charges, etc.
    emergency_contact: EmergencyContact,
    disciplinary_actions: Vec<String>, // Records of disciplinary actions
}

// Correctional Officer struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CorrectionalOfficer {
    id: u64,
    name: String,
    department_id: u64, // Reference to prison or department
    image: String,
    is_available: bool, // Whether the officer is on duty
}

// DisciplinaryAction struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct DisciplinaryAction {
    id: u64,
    inmate_id: u64,
    officer_id: u64,
    action: String, // Description of the action taken
    timestamp: u64, // Date and time of the action
}

// CommunicationLog struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CommunicationLog {
    id: u64,
    inmate_id: u64,
    officer_id: u64,
    message: String,
    timestamp: u64,
}

// CourtHearing struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CourtHearing {
    id: u64,
    inmate_id: u64,
    date: u64,       // Date of the hearing
    outcome: String, // Outcome of the hearing
}

// InmateRecord struct (used for storing the complete details of each inmate)
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct InmateRecord {
    id: u64,
    inmate_id: u64,
    disciplinary_actions: Vec<String>,
    court_hearings: Vec<u64>, // List of associated court hearings
    last_updated: u64,        // Timestamp for when the record was last updated
}

// Message enum
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
    PaymentFailed(String),
    PaymentCompleted(String),
}

// Payload Structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateInmatePayload {
    name: String,
    age: u64,
    gender: String,
    sentence_details: String,
    emergency_contact: EmergencyContact,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreatePrisonPayload {
    name: String,
    location: String,
    capacity: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateDisciplinaryActionPayload {
    inmate_id: u64,
    officer_id: u64,
    action: String,
}

// Implementing Storable for inmate
impl Storable for Inmate {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for inmate
impl BoundedStorable for Inmate {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for prison
impl Storable for Prison {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for prison
impl BoundedStorable for Prison {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for disciplinary action
impl Storable for DisciplinaryAction {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for disciplinary action
impl BoundedStorable for DisciplinaryAction {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for communication log
impl Storable for CommunicationLog {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for communication log
impl BoundedStorable for CommunicationLog {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for court hearing
impl Storable for CourtHearing {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for court hearing
impl BoundedStorable for CourtHearing {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for inmate record
impl Storable for InmateRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for inmate record
impl BoundedStorable for InmateRecord {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Memory management using thread_local
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static PRISONS: RefCell<StableBTreeMap<u64, Prison, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static INMATES: RefCell<StableBTreeMap<u64, Inmate, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static DISCIPLINARY_ACTIONS: RefCell<StableBTreeMap<u64, DisciplinaryAction, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static COMMUNICATION_LOGS: RefCell<StableBTreeMap<u64, CommunicationLog, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static COURT_HEARINGS: RefCell<StableBTreeMap<u64, CourtHearing, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));

    static INMATE_RECORDS: RefCell<StableBTreeMap<u64, InmateRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));
}

// Create Inmate function
#[ic_cdk::update]
fn create_inmate(payload: CreateInmatePayload) -> Result<Inmate, Message> {
    if payload.name.is_empty() || payload.sentence_details.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    let inmate_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let inmate = Inmate {
        id: inmate_id,
        name: payload.name,
        age: payload.age,
        gender: payload.gender,
        sentence_details: payload.sentence_details,
        emergency_contact: payload.emergency_contact,
        disciplinary_actions: Vec::new(),
    };

    INMATES.with(|inmates| {
        inmates.borrow_mut().insert(inmate_id, inmate.clone());
    });

    Ok(inmate)
}

// Create Prison function
#[ic_cdk::update]
fn create_prison(payload: CreatePrisonPayload) -> Result<Prison, Message> {
    if payload.name.is_empty() || payload.location.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    let prison_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let prison = Prison {
        id: prison_id,
        name: payload.name,
        location: payload.location,
        capacity: payload.capacity,
        inmates: Vec::new(),
    };

    PRISONS.with(|prisons| {
        prisons.borrow_mut().insert(prison_id, prison.clone());
    });

    Ok(prison)
}

// Function to create a disciplinary action
#[ic_cdk::update]
fn create_disciplinary_action(
    payload: CreateDisciplinaryActionPayload,
) -> Result<DisciplinaryAction, Message> {
    let disciplinary_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let disciplinary_action = DisciplinaryAction {
        id: disciplinary_id,
        inmate_id: payload.inmate_id,
        officer_id: payload.officer_id,
        action: payload.action,
        timestamp: time(),
    };

    DISCIPLINARY_ACTIONS.with(|actions| {
        actions
            .borrow_mut()
            .insert(disciplinary_id, disciplinary_action.clone());
    });

    Ok(disciplinary_action)
}

// Get Inmate by ID
#[ic_cdk::query]
fn get_inmate(id: u64) -> Result<Inmate, Message> {
    INMATES.with(|inmates| match inmates.borrow().get(&id) {
        Some(inmate) => Ok(inmate),
        None => Err(Message::NotFound("Inmate not found".to_string())),
    })
}

// Update Inmate Details
#[ic_cdk::update]
fn update_inmate(id: u64, payload: CreateInmatePayload) -> Result<Inmate, Message> {
    if payload.name.is_empty() || payload.sentence_details.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    INMATES.with(|inmates| {
        let mut inmates = inmates.borrow_mut();
        match inmates.get(&id) {
            Some(mut inmate) => {
                inmate.name = payload.name;
                inmate.age = payload.age;
                inmate.gender = payload.gender;
                inmate.sentence_details = payload.sentence_details;
                inmate.emergency_contact = payload.emergency_contact;
                inmates.insert(id, inmate.clone());
                Ok(inmate)
            }
            None => Err(Message::NotFound("Inmate not found".to_string())),
        }
    })
}

// Create Court Hearing
#[ic_cdk::update]
fn create_court_hearing(
    inmate_id: u64,
    date: u64,
    outcome: String,
) -> Result<CourtHearing, Message> {
    // Verify inmate exists
    match get_inmate(inmate_id) {
        Ok(_) => {
            let hearing_id = ID_COUNTER
                .with(|counter| {
                    let current_value = *counter.borrow().get();
                    counter.borrow_mut().set(current_value + 1)
                })
                .expect("Counter increment failed");

            let hearing = CourtHearing {
                id: hearing_id,
                inmate_id,
                date,
                outcome,
            };

            COURT_HEARINGS.with(|hearings| {
                hearings.borrow_mut().insert(hearing_id, hearing.clone());
            });

            // Update inmate record
            INMATE_RECORDS.with(|records| {
                let mut records = records.borrow_mut();
                match records.get(&inmate_id) {
                    Some(mut record) => {
                        record.court_hearings.push(hearing_id);
                        record.last_updated = time();
                        records.insert(inmate_id, record);
                    }
                    None => {
                        let record = InmateRecord {
                            id: inmate_id,
                            inmate_id,
                            disciplinary_actions: Vec::new(),
                            court_hearings: vec![hearing_id],
                            last_updated: time(),
                        };
                        records.insert(inmate_id, record);
                    }
                }
            });

            Ok(hearing)
        }
        Err(_) => Err(Message::NotFound("Inmate not found".to_string())),
    }
}

// Get Inmate Record with Full History
#[ic_cdk::query]
fn get_inmate_record(inmate_id: u64) -> Result<InmateRecord, Message> {
    INMATE_RECORDS.with(|records| match records.borrow().get(&inmate_id) {
        Some(record) => Ok(record),
        None => Err(Message::NotFound("Inmate record not found".to_string())),
    })
}

// Create Communication Log
#[ic_cdk::update]
fn create_communication_log(
    inmate_id: u64,
    officer_id: u64,
    message: String,
) -> Result<CommunicationLog, Message> {
    if message.is_empty() {
        return Err(Message::InvalidPayload(
            "Message cannot be empty".to_string(),
        ));
    }

    let log_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let log = CommunicationLog {
        id: log_id,
        inmate_id,
        officer_id,
        message,
        timestamp: time(),
    };

    COMMUNICATION_LOGS.with(|logs| {
        logs.borrow_mut().insert(log_id, log.clone());
    });

    Ok(log)
}

// Get Prison Details with Inmate Count
#[ic_cdk::query]
fn get_prison_details(prison_id: u64) -> Result<Prison, Message> {
    PRISONS.with(|prisons| match prisons.borrow().get(&prison_id) {
        Some(prison) => Ok(prison),
        None => Err(Message::NotFound("Prison not found".to_string())),
    })
}

// Assign Inmate to Prison
#[ic_cdk::update]
fn assign_inmate_to_prison(inmate_id: u64, prison_id: u64) -> Result<Prison, Message> {
    // Verify both inmate and prison exist
    match (get_inmate(inmate_id), get_prison_details(prison_id)) {
        (Ok(_), Ok(mut prison)) => PRISONS.with(|prisons| {
            let mut prisons = prisons.borrow_mut();
            if !prison.inmates.contains(&inmate_id) {
                if prison.inmates.len() as u64 >= prison.capacity {
                    return Err(Message::Error("Prison is at full capacity".to_string()));
                }
                prison.inmates.push(inmate_id);
                prisons.insert(prison_id, prison.clone());
            }
            Ok(prison)
        }),
        (Err(_), _) => Err(Message::NotFound("Inmate not found".to_string())),
        (_, Err(_)) => Err(Message::NotFound("Prison not found".to_string())),
    }
}

// Get All Disciplinary Actions for an Inmate
#[ic_cdk::query]
fn get_inmate_disciplinary_actions(inmate_id: u64) -> Result<Vec<DisciplinaryAction>, Message> {
    let mut actions = Vec::new();

    DISCIPLINARY_ACTIONS.with(|all_actions| {
        for (_, action) in all_actions.borrow().iter() {
            if action.inmate_id == inmate_id {
                actions.push(action);
            }
        }
    });

    if actions.is_empty() {
        Err(Message::NotFound(
            "No disciplinary actions found".to_string(),
        ))
    } else {
        Ok(actions)
    }
}

ic_cdk::export_candid!();
