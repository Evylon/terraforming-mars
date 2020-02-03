use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub id: String,
    pub cost: u32,
    pub card_type: CardType,
    pub deck: Deck,
    pub requirements: Requirements,
    pub tags: Vec<Tags>,
    pub production: Vec<Resource>,
    pub resources: Vec<Resource>,
    pub resources_on_card: HoldableResource,
    pub terraforming_effect: TerraformingEffect,
    pub interactions: Interactions,
    pub text: Text,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Requirements {
    pub global: GlobalRequirements,
    pub local: Vec<Tags>,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Tags {
    Science,
    Building,
    Space,
    Microbe,
    Plant,
    Animal,
    City,
    Earth,
    Jovian,
    Energy,
    Event,
    Special
}

impl fmt::Display for Tags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Tags {
    fn from_string(number: &String, tag: Tags) -> Vec<Tags> {
        match number.parse::<usize>() {
            Err(_e) => match number.as_ref() {
                "Ref" => vec![Tags::Special],
                _ => panic!("Cannot convert CSVCard tag to {} Tags::{}!", number, tag),
            }
            Ok(count) => vec![tag; count],
        }
    }

    // There are cards that require tiles in play, i.e. cities, colonies or greeneries
    // fn from_city_req(number: &String, card_name: &String) -> Vec<Tags> {
    //     match number.parse::<usize>() {
    //         Err(e) => match number.as_ref() {
    //             "Ref" => match card_name.as_ref() {
    //                 "Rad-Suits" => vec![Tags::City; 2],
    //                 "Martian Zoo" => vec![Tags::City; 2],
    //                 _ => panic!("Cannot convert CSVCard {} city tag to {} Tags::City!", card_name, number),
    //             }
    //             _ => panic!("Cannot convert CSVCard {} city tag to {} Tags::City!", card_name, number),
    //         }
    //         Ok(count) => vec![Tags::City; count],
    //     }
    // }
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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum CardType {
    Active,
    Automation,
    Corporation,
    Event,
    Prelude,
    Project,
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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Resource {
    MegaCredits(i32), Steel(i32), Titanium(i32), Plants(i32), Energy(i32), Heat(i32), Special,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Resource {
    pub fn from_string(number: &String, resource: Resource) -> Option<Resource> {
        match number.parse::<i32>() {
            Err(_e) => match number.as_ref() {
                "Ref" => Some(Resource::Special),
                "No" => None,
                "Floaters" => None, // TODO this is an extension feature
                _ => panic!("Cannot convert CSVCard resource to {} Resources::{}!", number, resource),
            }
            Ok(count) => match resource {
                Resource::MegaCredits(_) => Some(Resource::MegaCredits(count)),
                Resource::Steel(_) => Some(Resource::Steel(count)),
                Resource::Titanium(_) => Some(Resource::Titanium(count)),
                Resource::Plants(_) => Some(Resource::Plants(count)),
                Resource::Energy(_) => Some(Resource::Energy(count)),
                Resource::Heat(_) => Some(Resource::Heat(count)),
                Resource::Special => None,
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HoldableResource {
    Animals,
    Science,
    Microbes,
    TODO,
    None,
}

impl From<String> for HoldableResource {
    fn from(resource: String) -> Self {
        match resource.as_ref() {
            "Animals" => HoldableResource::Animals,
            "Science" => HoldableResource::Science,
            "Microbes" => HoldableResource::Microbes,
            "Ref" => HoldableResource::TODO,
            "Floaters" => HoldableResource::TODO,
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

impl From<CSVCard> for Card {
    fn from(csv_card: CSVCard) -> Self {
        Card {
            name: csv_card.card_name.to_owned(),
            id: match csv_card.id.len() {
                0 => csv_card.card_name,
                _ => csv_card.id
            },
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
                // There are cards that require tiles in play, i.e. cities, colonies or greeneries
                // TODO find a consistent way to deal with them
                local: vec![
                    vec![Tags::Science; csv_card.req_local_science as usize],
                    vec![Tags::Building; csv_card.req_local_building as usize],
                    vec![Tags::Space; csv_card.req_local_space as usize],
                    vec![Tags::Microbe; csv_card.req_local_microbe as usize],
                    vec![Tags::Plant; csv_card.req_local_plant as usize],
                    vec![Tags::Animal; csv_card.req_local_animal as usize],
                    Tags::from_string(&csv_card.req_local_city, Tags::City),
                    vec![Tags::Earth; csv_card.req_local_earth as usize],
                    vec![Tags::Jovian; csv_card.req_local_jovian as usize],
                    vec![Tags::Energy; csv_card.req_local_energy as usize],
                    vec![Tags::Special; match csv_card.req_local_other.as_ref() {"Ref" => 1, _ => 0}],
                ].concat(),
            },
            tags: vec![
                Tags::from_string(&csv_card.tag_science, Tags::Science),
                Tags::from_string(&csv_card.tag_building, Tags::Building),
                Tags::from_string(&csv_card.tag_space, Tags::Space),
                Tags::from_string(&csv_card.tag_microbe, Tags::Microbe),
                Tags::from_string(&csv_card.tag_plant, Tags::Plant),
                Tags::from_string(&csv_card.tag_animal, Tags::Animal),
                Tags::from_string(&csv_card.tag_city, Tags::City),
                Tags::from_string(&csv_card.tag_earth, Tags::Earth),
                Tags::from_string(&csv_card.tag_jovian, Tags::Jovian),
                Tags::from_string(&csv_card.tag_energy, Tags::Energy),
                Tags::from_string(&csv_card.tag_event, Tags::Event),
            ].concat(),
            production: vec![
                Resource::from_string(&csv_card.prod_megacredit, Resource::MegaCredits(0)).unwrap(),
                Resource::from_string(&csv_card.prod_steel, Resource::Steel(0)).unwrap(),
                Resource::from_string(&csv_card.prod_titanium, Resource::Titanium(0)).unwrap(),
                Resource::from_string(&csv_card.prod_plant, Resource::Plants(0)).unwrap(),
                Resource::from_string(&csv_card.prod_energy, Resource::Energy(0)).unwrap(),
                Resource::from_string(&csv_card.prod_heat, Resource::Heat(0)).unwrap(),
            ],
            resources: vec![
                Resource::from_string(&csv_card.inv_megacredit, Resource::MegaCredits(0)).unwrap(),
                Resource::from_string(&csv_card.inv_steel, Resource::Steel(0)).unwrap(),
                Resource::from_string(&csv_card.inv_titanium, Resource::Titanium(0)).unwrap(),
                Resource::from_string(&csv_card.inv_plant, Resource::Plants(0)).unwrap(),
                Resource::from_string(&csv_card.inv_energy, Resource::Energy(0)).unwrap(),
                Resource::from_string(&csv_card.inv_heat, Resource::Heat(0)).unwrap(),
            ],
            resources_on_card: HoldableResource::from(csv_card.other_resources_on_cards),
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
    req_local_science: u32,
    #[serde(rename = "Req: Building")]
    req_local_building: u32,
    #[serde(rename = "Req: Space")]
    req_local_space: u32,
    #[serde(rename = "Req: Microbe")]
    req_local_microbe: u32,
    #[serde(rename = "Req: Plant")]
    req_local_plant: u32,
    #[serde(rename = "Req: Animal")]
    req_local_animal: u32,
    #[serde(rename = "Req: City")]
    req_local_city: String,
    #[serde(rename = "Req: Earth")]
    req_local_earth: u32,
    #[serde(rename = "Req: Jovian")]
    req_local_jovian: u32,
    #[serde(rename = "Req: Energy")]
    req_local_energy: u32,
    #[serde(rename = "Req: Venus")]
    req_local_venus: u32,
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
