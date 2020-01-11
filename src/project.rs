use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;

use std::env;
use std::fs::File;
use std::fs;
use std::io::prelude::*;

use crate::game_state::GameState;
use crate::player::*;

pub fn convert_csv(csv_file: String, out_folder: String) {
    // load path
    let cwd = env::current_dir().unwrap();
    let mut project_list_path = cwd.to_owned();
    project_list_path.push(csv_file);
    let mut project_folder_path = cwd.to_owned();
    project_folder_path.push(out_folder);
    if !project_folder_path.exists() {
        fs::create_dir(project_folder_path.to_owned()).unwrap();
    }
    // load csv
    // FIXME column Req: Venus is duplicate. Good practice would be to rename headers after loading
    let mut rdr = csv::Reader::from_path(project_list_path).unwrap();
    for result in rdr.deserialize::<CSVProject>() {
        let project = Project::from(result.unwrap());
        let serialized = serde_json::to_string(&project).unwrap();
        // build output path
        let mut project_path = project_folder_path.to_owned();
        project_path.push(project.name);
        project_path.set_extension("json");
        let mut project_file = File::create(project_path).unwrap();
        project_file.write_all(serialized.as_bytes()).unwrap();
    }
}

pub struct CannotBuyCard;

impl Project {
    pub fn buy(self, buyer: &mut Player) -> Result<(), CannotBuyCard> {
        if buyer.inventory.megacredits < 3 {
            buyer.inventory.megacredits -= 3;
            buyer.hand.push(self);
            Ok(())
        } else {
            Err(CannotBuyCard)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub id: String,
    pub cost: u32,
    pub project_type: ProjectType,
    pub deck: Deck,
    pub requirements: Requirements,
    pub tags: Vec<Tags>,
    pub production: Production,
    pub resources: Resources,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
                _ => panic!("Cannot convert CSVProject tag to {} Tags::{}!", number, tag),
            }
            Ok(count) => vec![tag; count],
        }
    }

    // There are projects that require tiles in play, i.e. cities, colonies or greeneries
    // fn from_city_req(number: &String, project_name: &String) -> Vec<Tags> {
    //     match number.parse::<usize>() {
    //         Err(e) => match number.as_ref() {
    //             "Ref" => match project_name.as_ref() {
    //                 "Rad-Suits" => vec![Tags::City; 2],
    //                 "Martian Zoo" => vec![Tags::City; 2],
    //                 _ => panic!("Cannot convert CSVProject {} city tag to {} Tags::City!", project_name, number),
    //             }
    //             _ => panic!("Cannot convert CSVProject {} city tag to {} Tags::City!", project_name, number),
    //         }
    //         Ok(count) => vec![Tags::City; count],
    //     }
    // }
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
pub enum ProjectType {
    Active,
    Automation,
    Corporation,
    Event,
    Prelude,
}

impl From<String> for ProjectType {
    fn from(project_type: String) -> Self {
        match project_type.as_ref() {
            "Active" => ProjectType::Active,
            "Automation" => ProjectType::Automation,
            "Corporation" => ProjectType::Corporation,
            "Event" => ProjectType::Event,
            "Prelude" => ProjectType::Prelude,
            _ => ProjectType::Active // FIXME decide on behaviour here
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

impl From<CSVProject> for Project {
    fn from(csv_project: CSVProject) -> Self {
        Project {
            name: csv_project.project_name,
            id: csv_project.id,
            cost: match csv_project.cost.parse::<u32>() {
                Ok(number) => number,
                Err(_e) => 0
            },
            project_type: ProjectType::from(csv_project.project_type),
            deck: Deck::from(csv_project.deck),
            requirements: Requirements {
                global: GlobalRequirements {
                    min_temperature: csv_project.req_global_temperature,
                    max_temperature: csv_project.req_global_max_temperature,
                    min_oxygen: csv_project.req_global_oxygen,
                    max_oxygen: csv_project.req_global_max_oxygen,
                    min_ocean: csv_project.req_global_ocean,
                    max_ocean: csv_project.req_global_max_ocean,
                },
                // There are projects that require tiles in play, i.e. cities, colonies or greeneries
                // TODO find a consistent way to deal with them
                local: vec![
                    vec![Tags::Science; csv_project.req_local_science as usize],
                    vec![Tags::Building; csv_project.req_local_building as usize],
                    vec![Tags::Space; csv_project.req_local_space as usize],
                    vec![Tags::Microbe; csv_project.req_local_microbe as usize],
                    vec![Tags::Plant; csv_project.req_local_plant as usize],
                    vec![Tags::Animal; csv_project.req_local_animal as usize],
                    Tags::from_string(&csv_project.req_local_city, Tags::City),
                    vec![Tags::Earth; csv_project.req_local_earth as usize],
                    vec![Tags::Jovian; csv_project.req_local_jovian as usize],
                    vec![Tags::Energy; csv_project.req_local_energy as usize],
                    vec![Tags::Special; match csv_project.req_local_other.as_ref() {"Ref" => 1, _ => 0}],
                ].concat(),
            },
            tags: vec![
                Tags::from_string(&csv_project.tag_science, Tags::Science),
                Tags::from_string(&csv_project.tag_building, Tags::Building),
                Tags::from_string(&csv_project.tag_space, Tags::Space),
                Tags::from_string(&csv_project.tag_microbe, Tags::Microbe),
                Tags::from_string(&csv_project.tag_plant, Tags::Plant),
                Tags::from_string(&csv_project.tag_animal, Tags::Animal),
                Tags::from_string(&csv_project.tag_city, Tags::City),
                Tags::from_string(&csv_project.tag_earth, Tags::Earth),
                Tags::from_string(&csv_project.tag_jovian, Tags::Jovian),
                Tags::from_string(&csv_project.tag_energy, Tags::Energy),
                Tags::from_string(&csv_project.tag_event, Tags::Event),
            ].concat(),
            production: Production {
                megacredit: NumberOrRef::from(csv_project.prod_megacredit),
                steel: NumberOrRef::from(csv_project.prod_steel),
                titanium: NumberOrRef::from(csv_project.prod_titanium),
                plant: NumberOrRef::from(csv_project.prod_plant),
                energy: NumberOrRef::from(csv_project.prod_energy),
                heat: NumberOrRef::from(csv_project.prod_heat),
            },
            resources: Resources {
                megacredit: NumberOrRef::from(csv_project.inv_megacredit),
                steel: NumberOrRef::from(csv_project.inv_steel),
                titanium: NumberOrRef::from(csv_project.inv_titanium),
                plant: NumberOrRef::from(csv_project.inv_plant),
                energy: NumberOrRef::from(csv_project.inv_energy),
                heat: NumberOrRef::from(csv_project.inv_heat),
                other: BoolOrRef::from(csv_project.other_resources_on_projects),
            },
            terraforming_effect: TerraformingEffect {
                temperature: NumberOrRef::from(csv_project.temperature),
                oxygen: NumberOrRef::from(csv_project.oxygen),
                ocean: NumberOrRef::from(csv_project.ocean),
                tr: NumberOrRef::from(csv_project.tr),
                vp: NumberOrRef::from(csv_project.vp),
            },
            interactions: Interactions {
                tile_placement: BoolOrRef::from(csv_project.tile_colony_placement),
                num_actions_or_effect: NumberOrRef::from(csv_project.num_actions_and_or_effect),
                depends_on_opponents: BoolOrRef::from(csv_project.depends_on_opponents),
                affects_opponents: BoolOrRef::from(csv_project.affects_opponents),
                holds_resources: HoldableResource::from(csv_project.holds_resources),
            },
            text: Text {
                action_or_ongoing_effect_text: csv_project.action_or_on_going_effect_text,
                onetime_effect_text: csv_project.one_time_effect_text,
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CSVProject {
    #[serde(rename = "Card Name")]
    project_name: String,
    #[serde(rename = "Card #")]
    id: String,
    #[serde(rename = "Cost")]
    cost: String,
    #[serde(rename = "Card Type")]
    project_type: String,
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
    other_resources_on_projects: String,
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
