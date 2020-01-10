use serde::{Deserialize, Serialize};
use std::convert::From;

use std::env;
use std::fs::File;
use std::fs;
use std::io::prelude::*;

pub fn convert_csv(csv_file: String, out_folder: String) {
    // load path
    let cwd = env::current_dir().unwrap();
    let mut card_list_path = cwd.to_owned();
    card_list_path.push(csv_file);
    let mut card_folder_path = cwd.to_owned();
    card_folder_path.push(out_folder);
    if !card_folder_path.exists() {
        fs::create_dir(card_folder_path.to_owned()).unwrap();
    }
    // load csv
    // FIXME column Req: Venus is duplicate. Good practice would be to rename headers after loading
    let mut rdr = csv::Reader::from_path(card_list_path).unwrap();
    for result in rdr.deserialize::<CSVCard>() {
        let card = Card::from(result.unwrap());
        let serialized = serde_json::to_string(&card).unwrap();
        // build output path
        let mut card_path = card_folder_path.to_owned();
        card_path.push(card.name);
        card_path.set_extension("json");
        let mut card_file = File::create(card_path).unwrap();
        card_file.write_all(serialized.as_bytes()).unwrap();
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub id: String,
    pub cost: u32,
    pub card_type: CardType,
    pub deck: Deck,
    pub requirements: Requirements,
    pub tags: Tags,
    pub production: Production,
    pub resources: Resources,
    pub terraforming_effect: TerraformingEffect,
    pub interactions: Interactions,
    pub text: Text,
}

impl From<CSVCard> for Card {
    fn from(csv_card: CSVCard) -> Self {
        Card {
            name: csv_card.card_name,
            id: csv_card.id,
            cost: match csv_card.cost.parse::<u32>() {
                Ok(number) => number,
                Err(_e) => 0
            },
            card_type: CardType::from(csv_card.card_type),
            deck: Deck::from(csv_card.deck),
            requirements: Requirements {
                global: GlobalRequirements {
                    min_temperature: csv_card.req_global_temperature,
                    max_temperature: csv_card.req_global_max_temperature,
                    min_oxygen: csv_card.req_global_oxygen,
                    max_oxygen: csv_card.req_global_max_oxygen,
                    min_ocean: csv_card.req_global_ocean,
                    max_ocean: csv_card.req_global_max_ocean,
                },
                local: LocalRequirements {
                    science: csv_card.req_local_science,
                    building: csv_card.req_local_building,
                    space: csv_card.req_local_space,
                    microbe: csv_card.req_local_microbe,
                    plant: csv_card.req_local_plant,
                    animal: csv_card.req_local_animal,
                    city: NumberOrRef::from(csv_card.req_local_city),
                    earth: csv_card.req_local_earth,
                    jovian: csv_card.req_local_jovian,
                    energy: csv_card.req_local_energy,
                    other: BoolOrRef::from(csv_card.req_local_other),
                },
            },
            tags: Tags {
                science: NumberOrRef::from(csv_card.tag_science),
                building: NumberOrRef::from(csv_card.tag_building),
                space: NumberOrRef::from(csv_card.tag_space),
                microbe: NumberOrRef::from(csv_card.tag_microbe),
                plant: NumberOrRef::from(csv_card.tag_plant),
                animal: NumberOrRef::from(csv_card.tag_animal),
                city: NumberOrRef::from(csv_card.tag_city),
                earth: NumberOrRef::from(csv_card.tag_earth),
                jovian: NumberOrRef::from(csv_card.tag_jovian),
                energy: NumberOrRef::from(csv_card.tag_energy),
                event: NumberOrRef::from(csv_card.tag_event),
            },
            production: Production {
                megacredit: NumberOrRef::from(csv_card.prod_megacredit),
                steel: NumberOrRef::from(csv_card.prod_steel),
                titanium: NumberOrRef::from(csv_card.prod_titanium),
                plant: NumberOrRef::from(csv_card.prod_plant),
                energy: NumberOrRef::from(csv_card.prod_energy),
                heat: NumberOrRef::from(csv_card.prod_heat),
            },
            resources: Resources {
                megacredit: NumberOrRef::from(csv_card.inv_megacredit),
                steel: NumberOrRef::from(csv_card.inv_steel),
                titanium: NumberOrRef::from(csv_card.inv_titanium),
                plant: NumberOrRef::from(csv_card.inv_plant),
                energy: NumberOrRef::from(csv_card.inv_energy),
                heat: NumberOrRef::from(csv_card.inv_heat),
                other: BoolOrRef::from(csv_card.other_resources_on_cards),
            },
            terraforming_effect: TerraformingEffect {
                temperature: NumberOrRef::from(csv_card.temperature),
                oxygen: NumberOrRef::from(csv_card.oxygen),
                ocean: NumberOrRef::from(csv_card.ocean),
                tr: NumberOrRef::from(csv_card.tr),
                vp: NumberOrRef::from(csv_card.vp),
            },
            interactions: Interactions {
                tile_placement: BoolOrRef::from(csv_card.tile_colony_placement),
                num_actions_or_effect: NumberOrRef::from(csv_card.num_actions_and_or_effect),
                depends_on_opponents: BoolOrRef::from(csv_card.depends_on_opponents),
                affects_opponents: BoolOrRef::from(csv_card.affects_opponents),
                holds_resources: HoldableResource::from(csv_card.holds_resources),
            },
            text: Text {
                action_or_ongoing_effect_text: csv_card.action_or_on_going_effect_text,
                onetime_effect_text: csv_card.one_time_effect_text,
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Requirements {
    pub global: GlobalRequirements,
    pub local: LocalRequirements,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobalRequirements {
    pub min_temperature: i32,
    pub max_temperature: i32,
    pub min_oxygen: u32,
    pub max_oxygen: u32,
    pub min_ocean: u32,
    pub max_ocean: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalRequirements {
    pub science: i32,
    pub building: i32,
    pub space: i32,
    pub microbe: i32,
    pub plant: i32,
    pub animal: i32,
    pub city: NumberOrRef,
    pub earth: i32,
    pub jovian: i32,
    pub energy: i32,
    pub other: BoolOrRef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tags {
    pub science: NumberOrRef,
    pub building: NumberOrRef,
    pub space: NumberOrRef,
    pub microbe: NumberOrRef,
    pub plant: NumberOrRef,
    pub animal: NumberOrRef,
    pub city: NumberOrRef,
    pub earth: NumberOrRef,
    pub jovian: NumberOrRef,
    pub energy: NumberOrRef,
    pub event: NumberOrRef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Production {
    pub megacredit: NumberOrRef,
    pub steel: NumberOrRef,
    pub titanium: NumberOrRef,
    pub plant: NumberOrRef,
    pub energy: NumberOrRef,
    pub heat: NumberOrRef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resources {
    pub megacredit: NumberOrRef,
    pub steel: NumberOrRef,
    pub titanium: NumberOrRef,
    pub plant: NumberOrRef,
    pub energy: NumberOrRef,
    pub heat: NumberOrRef,
    pub other: BoolOrRef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerraformingEffect {
    pub temperature: NumberOrRef,
    pub oxygen: NumberOrRef,
    pub ocean: NumberOrRef,
    pub tr: NumberOrRef,
    pub vp: NumberOrRef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Interactions {
    pub tile_placement: BoolOrRef,
    pub num_actions_or_effect: NumberOrRef,
    pub depends_on_opponents: BoolOrRef,
    pub affects_opponents: BoolOrRef,
    pub holds_resources: HoldableResource,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Text {
    pub action_or_ongoing_effect_text: String,
    pub onetime_effect_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CardType {
    Active,
    Automation,
    Corporation,
    Event,
    Prelude,
}

impl From<String> for CardType {
    fn from(card_type: String) -> Self {
        match card_type.as_ref() {
            "Active" => CardType::Active,
            "Automation" => CardType::Automation,
            "Corporation" => CardType::Corporation,
            "Event" => CardType::Event,
            "Prelude" => CardType::Prelude,
            _ => CardType::Active // FIXME decide on behaviour here
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Deck {
    Basic,
    Colonies,
    Corporate,
    Prelude,
    Promo,
    Venus,
}

impl From<String> for Deck {
    fn from(deck: String) -> Self {
        match deck.as_ref() {
            "Basic" => Deck::Basic,
            "Colonies" => Deck::Colonies,
            "Corporate" => Deck::Corporate,
            "Prelude" => Deck::Prelude,
            "Promo" => Deck::Promo,
            "Venus" => Deck::Venus,
            _ => Deck::Basic // FIXME decide on behaviour here
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HoldableResource {
    Animals,
    Science,
    Microbes,
    None,
}

impl From<String> for HoldableResource {
    fn from(resource: String) -> Self {
        match resource.as_ref() {
            "Animals" => HoldableResource::Animals,
            "Science" => HoldableResource::Science,
            "Microbes" => HoldableResource::Microbes,
            "No" => HoldableResource::None,
            _ => HoldableResource::None // FIXME decide on behaviour here
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NumberOrRef {
    Number(i32),
    Ref,
}

impl From<String> for NumberOrRef {
    fn from(number: String) -> Self {
        match number.parse::<i32>() {
            Ok(number) => NumberOrRef::Number(number),
            Err(_e) => NumberOrRef::Ref, // FIXME this is only for num_actions_or_effect, create separate solution
        }
    }
}

// TODO create types for common effects
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BoolOrRef {
    No,
    Ref,
}

impl From<String> for BoolOrRef {
    fn from(val: String) -> Self {
        match val.as_ref() {
            "No" => BoolOrRef::No,
            "Ref" => BoolOrRef::Ref,
            _ => BoolOrRef::No // FIXME decide on behaviour here
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CSVCard {
    #[serde(rename = "Card Name")]
    card_name: String,
    #[serde(rename = "Card #")]
    id: String,
    #[serde(rename = "Cost")]
    cost: String,
    #[serde(rename = "Card Type")]
    card_type: String,
    #[serde(rename = "Deck")]
    deck: String,
    #[serde(rename = "Req: Min Temperature")]
    req_global_temperature: i32,
    #[serde(rename = "Req: Min Oxygen")]
    req_global_oxygen: u32,
    #[serde(rename = "Req: Min Ocean")]
    req_global_ocean: u32,
    #[serde(rename = "Req: Min Venus")]
    req_global_venus: i32,
    #[serde(rename = "Req: Max Temperature")]
    req_global_max_temperature: i32,
    #[serde(rename = "Req: Max Oxygen")]
    req_global_max_oxygen: u32,
    #[serde(rename = "Req: Max Ocean")]
    req_global_max_ocean: u32,
    #[serde(rename = "Req: Max Venus")]
    req_global_max_venus: i32,
    #[serde(rename = "Req: Science")]
    req_local_science: i32,
    #[serde(rename = "Req: Building")]
    req_local_building: i32,
    #[serde(rename = "Req: Space")]
    req_local_space: i32,
    #[serde(rename = "Req: Microbe")]
    req_local_microbe: i32,
    #[serde(rename = "Req: Plant")]
    req_local_plant: i32,
    #[serde(rename = "Req: Animal")]
    req_local_animal: i32,
    #[serde(rename = "Req: City")]
    req_local_city: String,
    #[serde(rename = "Req: Earth")]
    req_local_earth: i32,
    #[serde(rename = "Req: Jovian")]
    req_local_jovian: i32,
    #[serde(rename = "Req: Energy")]
    req_local_energy: i32,
    #[serde(rename = "Req: Venus")]
    req_local_venus: i32,
    #[serde(rename = "Req: Other")]
    req_local_other: String,
    #[serde(rename = "Tag: Science")]
    tag_science: String,
    #[serde(rename = "Tag: Building")]
    tag_building: String,
    #[serde(rename = "Tag: Space")]
    tag_space: String,
    #[serde(rename = "Tag: Microbe")]
    tag_microbe: String,
    #[serde(rename = "Tag: Plant")]
    tag_plant: String,
    #[serde(rename = "Tag: Animal")]
    tag_animal: String,
    #[serde(rename = "Tag: City")]
    tag_city: String,
    #[serde(rename = "Tag: Earth")]
    tag_earth: String,
    #[serde(rename = "Tag: Jovian")]
    tag_jovian: String,
    #[serde(rename = "Tag: Energy")]
    tag_energy: String,
    #[serde(rename = "Tag: Venus")]
    tag_venus: String,
    #[serde(rename = "Tag: Event")]
    tag_event: String,
    #[serde(rename = "Prod: Megacredit")]
    prod_megacredit: String,
    #[serde(rename = "Prod: Steel")]
    prod_steel: String,
    #[serde(rename = "Prod: Titanium")]
    prod_titanium: String,
    #[serde(rename = "Prod: Plant")]
    prod_plant: String,
    #[serde(rename = "Prod: Energy")]
    prod_energy: String,
    #[serde(rename = "Prod: Heat")]
    prod_heat: String,
    #[serde(rename = "Inv: Megacredit")]
    inv_megacredit: String,
    #[serde(rename = "Inv: Steel")]
    inv_steel: String,
    #[serde(rename = "Inv: Titanium")]
    inv_titanium: String,
    #[serde(rename = "Inv: Plant")]
    inv_plant: String,
    #[serde(rename = "Inv: Energy")]
    inv_energy: String,
    #[serde(rename = "Inv: Heat")]
    inv_heat: String,
    #[serde(rename = "Other (Resources on Cards)")]
    other_resources_on_cards: String,
    #[serde(rename = "Temperature")]
    temperature: String,
    #[serde(rename = "Oxygen")]
    oxygen: String,
    #[serde(rename = "Ocean")]
    ocean: String,
    #[serde(rename = "Venus")]
    venus: String,
    #[serde(rename = "TR")]
    tr: String,
    #[serde(rename = "VP")]
    vp: String,
    #[serde(rename = "Tile/Colony Placement")]
    tile_colony_placement: String,
    #[serde(rename = "# Actions and/or Effect")]
    num_actions_and_or_effect: String,
    #[serde(rename = "Depends on opponents")]
    depends_on_opponents: String,
    #[serde(rename = "Affects opponents")]
    affects_opponents: String,
    #[serde(rename = "Holds Resources")]
    holds_resources: String,
    #[serde(rename = "Action or On-going Effect text")]
    action_or_on_going_effect_text: String,
    #[serde(rename = "One time Effect Text")]
    one_time_effect_text: String,
}
