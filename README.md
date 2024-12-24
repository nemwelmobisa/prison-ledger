# Prison Management System

A secure and efficient prison management system built on the Internet Computer (ICP) blockchain platform. This system provides comprehensive tools for managing inmates, prisons, correctional officers, and various administrative tasks in a correctional facility.

## Features

### Core Management
- Prison facility management
- Inmate records and tracking
- Correctional officer administration
- Emergency contact information
- Disciplinary action tracking

### Record Keeping
- Court hearing schedules and outcomes
- Communication logs
- Inmate records with complete history
- Disciplinary action records

### Security and Compliance
- Secure data storage using IC's stable storage
- Immutable record keeping
- Access control and authentication
- Capacity management for facilities

## Technical Architecture

### Data Structures
- **Prison**: Manages facility information and inmate capacity
- **Inmate**: Stores prisoner information and status
- **CorrectionalOfficer**: Manages officer information and availability
- **DisciplinaryAction**: Tracks disciplinary measures
- **CommunicationLog**: Records official communications
- **CourtHearing**: Manages court appearances and outcomes
- **InmateRecord**: Maintains comprehensive inmate history

### Storage Implementation
- Uses `ic_stable_structures` for persistent storage
- Implements `BoundedStorable` for efficient data management
- Thread-local storage for concurrent access

## Installation

1. Install the DFINITY Canister SDK:
```bash
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

2. Clone the repository:
```bash
git clone [repository-url]
cd prison-management-system
```

3. Deploy the canister:
```bash
dfx start --background
dfx deploy
```

## Usage Examples

### Creating a New Prison
```rust
let prison_payload = CreatePrisonPayload {
    name: "Central Correctional Facility".to_string(),
    location: "123 Security Ave".to_string(),
    capacity: 500,
};

match create_prison(prison_payload) {
    Ok(prison) => println!("Prison created with ID: {}", prison.id),
    Err(e) => println!("Error: {:?}", e),
}
```

### Managing Inmates
```rust
// Create new inmate
let inmate_payload = CreateInmatePayload {
    name: "John Doe".to_string(),
    age: 30,
    gender: "Male".to_string(),
    sentence_details: "5 years for fraud".to_string(),
    emergency_contact: EmergencyContact {
        name: "Jane Doe".to_string(),
        phone_number: "555-0123".to_string(),
        relationship: "Spouse".to_string(),
    },
};

let inmate = create_inmate(inmate_payload)?;

// Assign to prison
assign_inmate_to_prison(inmate.id, prison_id)?;
```

### Recording Disciplinary Actions
```rust
let action_payload = CreateDisciplinaryActionPayload {
    inmate_id: 1,
    officer_id: 1,
    action: "Violation of facility rules".to_string(),
};

create_disciplinary_action(action_payload)?;
```

## API Reference

### Prison Management
- `create_prison(payload: CreatePrisonPayload) -> Result<Prison, Message>`
- `get_prison_details(prison_id: u64) -> Result<Prison, Message>`
- `assign_inmate_to_prison(inmate_id: u64, prison_id: u64) -> Result<Prison, Message>`

### Inmate Management
- `create_inmate(payload: CreateInmatePayload) -> Result<Inmate, Message>`
- `get_inmate(id: u64) -> Result<Inmate, Message>`
- `update_inmate(id: u64, payload: CreateInmatePayload) -> Result<Inmate, Message>`
- `get_inmate_record(inmate_id: u64) -> Result<InmateRecord, Message>`

### Record Keeping
- `create_court_hearing(inmate_id: u64, date: u64, outcome: String) -> Result<CourtHearing, Message>`
- `create_communication_log(inmate_id: u64, officer_id: u64, message: String) -> Result<CommunicationLog, Message>`
- `get_inmate_disciplinary_actions(inmate_id: u64) -> Result<Vec<DisciplinaryAction>, Message>`

## Error Handling

The system uses a Message enum for error handling:
```rust
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
    PaymentFailed(String),
    PaymentCompleted(String),
}
```

## Security Considerations

1. **Data Privacy**: All sensitive information is stored securely on the Internet Computer blockchain
2. **Access Control**: Implement proper authentication and authorization before deployment
3. **Audit Trail**: All actions are logged with timestamps and officer IDs
4. **Capacity Management**: Automatic checks prevent prison overcrowding

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Internet Computer (ICP) Platform
- DFINITY Foundation
- Rust Community
- Contributors and maintainers

## Support

For support, please open an issue in the repository or contact the maintenance team.