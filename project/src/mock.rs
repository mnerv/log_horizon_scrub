use crate::admin_service::*;
use crate::command::*;
use crate::common_service::*;
use crate::customer_service::*;
use std::error::Error;

fn admin_mock() -> Result<(), Box<dyn Error>> {
    let mut admin = RegisterAdminCommand {
        email: "admin".to_string(),
        password: "admin".to_string(),
    }
    .run()?;

    admin = RegisterAdminCommand {
        email: "eric@hopestore.se".to_string(),
        password: "hello".to_string(),
    }
    .run()?;

    let elec = AddSupplierCommand {
        name: "Electrokit AB".to_string(),
        description: "Elektronikkomponenter, byggsatser, mätinstrument och tillbehör.".to_string(),
        org_num: "556667-9683".to_string(),
        street: "Västkustvägen 7".to_string(),
        postcode: "211 24".to_string(),
        city: "Malmö".to_string(),
        country: "Sverige".to_string(),
        telephone: "040-298760".to_string(),
    }
    .run(&mut admin)?;

    admin = RegisterAdminCommand {
        email: "pratchaya@hopestore.se".to_string(),
        password: "hello".to_string(),
    }
    .run()?;

    let apple = AddSupplierCommand {
        name: "Apple".to_string(),
        description: "Hardware and softwares".to_string(),
        org_num: "333333-6666".to_string(),
        street: "One Apple Park Way".to_string(),
        postcode: "95014".to_string(),
        city: "Cupertino, CA".to_string(),
        country: "Sverige".to_string(),
        telephone: "(408) 996–1010".to_string(),
    }
    .run(&mut admin)?;

    let ikea = AddSupplierCommand {
        name: "IKEA".to_string(),
        description: "Hemma".to_string(),
        org_num: "777777-6666".to_string(),
        street: "Kulthusgatan 1".to_string(),
        postcode: "215 86".to_string(),
        city: "Malmö".to_string(),
        country: "Sverige".to_string(),
        telephone: "040-000000".to_string(),
    }
    .run(&mut admin)?;

    AddProductCommand {
        supplier_id: elec.id,
        name: "Raspberry Pi Compute Module 4 – 4GB + 32GB WiFi".to_string(),
        description: "Raspberry Pi Compute Module 4 är kraften i en Raspberry Pi 4 i en formfaktor som passar perfekt för inbyggda system.".to_string(),
        price: 909.00,
        quantity: 100,
    }
    .run(&mut admin)?;

    AddProductCommand {
        supplier_id: elec.id,
        name: "Raspberry Pi 4 Model B/8GB".to_string(),
        description:
            "Den nya 8GB-modellen ger förutom dubbelt så mycket mot den förra flaggskeppsmodellen."
                .to_string(),
        price: 1_099.00,
        quantity: 32,
    }
    .run(&mut admin)?;

    AddProductCommand {
        supplier_id: elec.id,
        name: "Raspberry Pi Pico W".to_string(),
        description: "Den populära enkortsdatorn Raspberry Pi Pico har nu försetts med Wifi."
            .to_string(),
        price: 98.00,
        quantity: 1_013,
    }
    .run(&mut admin)?;

    AddProductCommand {
        supplier_id: ikea.id,
        name: "STRÅLA".to_string(),
        description: "Lampskärm, mässingsfärgad/rand".to_string(),
        price: 59.00,
        quantity: 10,
    }
    .run(&mut admin)?;

    AddProductCommand {
        supplier_id: ikea.id,
        name: "EKET".to_string(),
        description: "Skåp, gråturkos, 35x25x35 cm".to_string(),
        price: 95.00,
        quantity: 50,
    }
    .run(&mut admin)?;

    AddProductCommand {
        supplier_id: apple.id,
        name: "iPhone 14 Pro 256GB".to_string(),
        description: "Ett magiskt nytt sätt att samspela med din iPhone.".to_string(),
        price: 15_000.00,
        quantity: 64,
    }
    .run(&mut admin)?;

    Ok(())
}

pub fn mock_data() -> Result<(), Box<dyn Error>> {
    admin_mock()?;
    Ok(())
}
