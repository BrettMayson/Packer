pub struct Map {
    world: String,
    name: Option<String>,
}

pub fn all() -> Vec<Map> {
    let mut maps = Vec::new();
    maps.append(&mut vanilla());
    maps.append(&mut contact());
    maps.append(&mut western_sahara());
    maps.append(&mut cup_maps());
    maps
}

pub fn vanilla() -> Vec<Map> {
    vec![
        Map {
            world: String::from("Altis"),
            name: Some(String::from("Altis")),
        },
        Map {
            world: String::from("Stratis"),
            name: Some(String::from("Stratis")),
        },
        Map {
            world: String::from("Malden"),
            name: Some(String::from("Malden")),
        },
        Map {
            world: String::from("Tanoa"),
            name: Some(String::from("Tanoa")),
        },
        Map {
            world: String::from("VR"),
            name: Some(String::from("Virtual Reality")),
        },
    ]
}

pub fn contact() ->  Vec<Map> {
    vec![
        Map {
            world: String::from("Livonia"),
            name: Some(String::from("Livonia")),
        },
    ]
}

pub fn western_sahara() -> Vec<Map> {
    vec![
        Map {
            world: String::from("SefrouRamal"),
            name: Some(String::from("Sefrou Ramal")),
        },
    ]
}

pub fn cup_maps() -> Vec<Map> {
    vec![
        Map {
            world: String::from("Bootcamp_ACR"),
            name: Some(String::from("Bukovina")),
        },
        Map {
            world: String::from("chernarus"),
            name: Some(String::from("Chernarus")),
        },
        Map {
            world: String::from("chernarus_summer"),
            name: Some(String::from("Chernarus (Summer)")),
        },
        Map {
            world: String::from("Chernarus_Winter"),
            name: Some(String::from("Chernarus (Winter)")),
        },
        Map {
            world: String::from("Desert_E"),
            name: Some(String::from("Desert")),
        },
        Map {
            world: String::from("intro"),
            name: Some(String::from("Rahmadi")),
        },
        Map {
            world: String::from("Mountains_ACR"),
            name: Some(String::from("Takistan Mountains")),
        },
        Map {
            world: String::from("porto"),
            name: Some(String::from("Porto")),
        },
        Map {
            world: String::from("ProvingGrounds_PMC"),
            name: Some(String::from("Proving Grounds")),
        },
        Map {
            world: String::from("sara"),
            name: Some(String::from("Sahrani")),
        },
        Map {
            world: String::from("sara_dbe1"),
            name: Some(String::from("United Sahrani")),
        },
        Map {
            world: String::from("saralite"),
            name: Some(String::from("Southern Sahrani")),
        },
        Map {
            world: String::from("Shapur_BAF"),
            name: Some(String::from("Shapur")),
        },
        Map {
            world: String::from("takistan"),
            name: Some(String::from("Takistan")),
        },
        Map {
            world: String::from("utes"),
            name: Some(String::from("Utes")),
        },
        Map {
            world: String::from("Woodland_ACR"),
            name: Some(String::from("Bystrica")),
        },
        Map {
            world: String::from("zargabad"),
            name: Some(String::from("Zargabad")),
        },
    ]
}
