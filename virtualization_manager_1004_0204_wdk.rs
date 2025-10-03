use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::http::Status;
use rocket::serde::json::serde_json;
use rocket::response::status;
use std::collections::HashMap;

#[macro_use] extern crate rocket;

// Define a structure for VirtualMachine
#[derive(Serialize, Deserialize, Clone, Debug)]
struct VirtualMachine {
    uuid: String,
    name: String,
    status: String,
    // Add other properties as needed
}

// Define an error type for VirtualMachine errors
#[derive(Debug)]
enum VirtualMachineError {
    NotFound(String),
    InvalidInput(String),
    // Add other error types as needed
}

// Implement the error handling for VirtualMachineError
impl<'r> status::Responder<'r, 'static> for VirtualMachineError {
    fn respond_to(self, _: &'r rocket::Request) -> status::Response<'static> {
        match self {
            VirtualMachineError::NotFound(uuid) => {
                status::Custom(Status::NotFound, Json(format!("VM with UUID {} not found", uuid)))
            },
            VirtualMachineError::InvalidInput(message) => {
                status::Custom(Status::BadRequest, Json(message))
            },
        }
    }
}

// Define a service structure that will handle our business logic
struct VirtualizationService {
    machines: HashMap<String, VirtualMachine>,
}

impl VirtualizationService {
    fn new() -> Self {
        VirtualizationService {
            machines: HashMap::new(),
        }
    }

    // Function to add a new VirtualMachine
    fn create_vm(&mut self, vm: VirtualMachine) -> Result<&'_ VirtualMachine, VirtualMachineError> {
        self.machines.insert(vm.uuid.clone(), vm);
        Ok(self.machines.get(&vm.uuid).unwrap())
    }

    // Function to get a VirtualMachine by UUID
    fn get_vm(&self, uuid: &str) -> Result<&VirtualMachine, VirtualMachineError> {
        self.machines.get(uuid).ok_or(VirtualMachineError::NotFound(uuid.to_string()))
    }

    // Function to list all VirtualMachines
    fn list_vms(&self) -> Vec<&VirtualMachine> {
        self.machines.values().collect()
    }

    // Add other functions as needed
}

#[get("/vms")]
fn list_vmachines(service: rocket::State<VirtualizationService>) -> Json<Vec<VirtualMachine>> {
    Json(service.list_vms().into_iter().cloned().collect())
}

#[get("/vms/<uuid>")]
fn get_vmachine(uuid: String, service: rocket::State<VirtualizationService>) -> Result<Json<VirtualMachine>, VirtualMachineError> {
    match service.get_vm(&uuid) {
        Ok(vm) => Ok(Json(vm.clone())),
        Err(e) => Err(e),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![list_vmachines, get_vmachine])
        .manage(VirtualizationService::new())
}
