pub mod octopus_handler {
    use axum::Json;
    use serde_derive::{Deserialize, Serialize};
    use std::env;

    const OCTO_API: &str = "https://api.octopus.energy/v1";

    pub async fn main() -> &'static str {
        "Hello from octopus handler"
    }

    #[derive(Serialize)]
    pub struct GasAbout {
        mprn: String,
        serial_number: String,
    }

    #[derive(Serialize)]
    pub struct ElectricAbout {
        mprn: String,
        serial_number: String,
    }

    #[derive(Serialize)]
    pub struct OctopusAbout {
        gas: GasAbout,
        electricity: ElectricAbout,
    }

    /**
     * Returns the current data stored about the octopus account
     */
    // TODO: fix panic and better handle errors
    pub async fn about() -> Json<OctopusAbout> {
        let gas_about = GasAbout {
            mprn: env::var("OCTOPUS_GAS_MPRN").unwrap(),
            serial_number: env::var("OCOTPUS_GAS_SERIAL").unwrap(),
        };

        let electric_about = ElectricAbout {
            mprn: env::var("OCTOPUS_ELECTRIC_MPRN").unwrap(),
            serial_number: env::var("OCTOPUS_ELECTRIC_SERIAL").unwrap(),
        };

        let octopus_about = OctopusAbout {
            gas: gas_about,
            electricity: electric_about,
        };

        return Json(octopus_about);
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
