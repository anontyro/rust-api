pub mod octopus_handler {
    pub async fn main() -> &'static str {
        "Hello from octopus handler"
    }

    // TODO: get the latest data from octopus energy for gas consumption, need to paginated
    pub async fn get_gas_consumption() -> &'static str {
        "Hello from getGasConsumption"
    }

    // TODO: get the latest data from octopus energy for electricity consumption, need to paginated
    pub async fn get_electricity_consumption() -> &'static str {
        "Hello from getElectricityConsumption"
    }

}


// IDEAS

// endpoint that will call to get the latest data from octopus energy
// should check if the data is already in the database using timestamp probably