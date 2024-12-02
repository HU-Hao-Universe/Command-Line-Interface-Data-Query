use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io;
use std::io::Read;
use chrono::{NaiveDate, Duration};
#[allow(non_snake_case)]

//Assemblies JSON
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "ASM")]
    pub ASM: Vec<ASM>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ASM {
    #[serde(rename = "SerialNumber")]
    pub serial_number: String,
    #[serde(rename = "BuiltDate")]
    pub built_date: i64,
    #[serde(rename = "BuiltBy")]
    pub built_by: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "SalesOrder")]
    pub sales_order: String,
}

//DWE Json
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    #[serde(rename = "DWE")]
    pub DWE: Vec<DWE>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DWE {
    #[serde(rename = "Enclosure SN")]
    pub enclosure_sn: String,
    #[serde(rename = "Drive SN")]
    pub drive_sn: String,
    #[serde(rename = "Drive Manufacturer")]
    pub drive_manufacturer: String,
    #[serde(rename = "Model")]
    pub drive_model: String,
    #[serde(rename = "Part Number")]
    pub drive_part: String,
}

//Zendesk JSON
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root3 {
    #[serde(rename = "ZEN")]
    pub zen: Vec<ZEN>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZEN {
    #[serde(rename = "RMA")]
    pub rma: i64,
    #[serde(rename = "Serial")]
    pub serial: String,
    #[serde(rename = "Drive")]
    pub drive: String,
    #[serde(rename = "Assembler")]
    pub assembler: String,
    #[serde(rename = "OldDiagnosis")]
    pub old_diagnosis: String,
    #[serde(rename = "NewDiagnosis")]
    pub new_diagnosis: String,
}

fn main(){
    println!("Glyph Database Started. Type Q to quit.");
    println!("Loading Assemblies Database.");
    let mut file = File::open("ASM.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let assemblers: Root = serde_json::from_str(&data).unwrap();
    println!("Assemblies Database Loaded.");

    println!("Loading Drives with Enclosures Database.");
    let mut file2 = File::open("DWE.json").unwrap();
    let mut data2 = String::new();
    file2.read_to_string(&mut data2).unwrap();
    let dwe: Root2 = serde_json::from_str(&data2).unwrap();
    println!("Drives with Enclosures Database Loaded.");

    println!("Loading Zendesk Database.");
    let mut file3 = File::open("ZEN.json").unwrap();
    let mut data3 = String::new();
    file3.read_to_string(&mut data3).unwrap();
    let zen: Root3 = serde_json::from_str(&data3).unwrap();
    println!("Zendesk Database Loaded.");

    println!("Formatting serialized JSON data.");
    //Formatting Assemblies.
    let mut xd = String::from(format!("{:?}", assemblers));
    xd = xd.replace("{", "");
    xd = xd.replace("}", "");
    xd = xd.replace(",", "");
    xd = xd.to_uppercase();
    
    //Formatting DWE.
    let mut nigger = String::from(format!("{:?}", dwe));
    nigger = nigger.replace("{", "");
    nigger = nigger.replace("}", "");
    nigger = nigger.replace(",", "");
    nigger = nigger.to_uppercase();    

    //Formatting Zendesk
    let mut cringe = String::from(format!("{:?}", zen));
    cringe = cringe.replace("{", "");
    cringe = cringe.replace("}", "");
    cringe = cringe.replace(",", "");
    cringe = cringe.to_uppercase();

    println!("JSON data successfully formatted.");

    let mut owari = false;
    let mut ser1 = String::new();
    let mut prevresults = String::new();

    println!("Please enter search criteria:");
    while !owari {
        let mut pieces = nigger.split("DWE");
        let mut parts = xd.split("ASM");
        let mut bits = cringe.split("ZEN");
        //Input Serial
        io::stdin().read_line(&mut ser1);
        ser1 = ser1.replace("\r\n", ""); //For Windows
        //ser1 = ser1.replace("\n", ""); //For Linux    
        ser1 = ser1.to_uppercase();

        if ser1 == "Q" {
            owari = true;
            break;
        }

        if ser1.starts_with("#"){
            ser1 = ser1.replace("#", "");
            let mut thunking = prevresults.split("  ");
            for part in thunking {
                if part.contains(&ser1){
                    println!("{}", part);
                }
            }
        }
        else if ser1.starts_with("$"){
            ser1 = ser1.replace("$", "");
            let mut date = ser1.parse::<i64>().unwrap();
            println!("{}", date);
            date = date/86400000+25569;
            println!("{}", date);
            let dt = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap() + Duration::days(date);
            println!("{}", dt);
        }
        else {
        prevresults = "".to_string();
        
        //Assemblies shenanigans
        for part in parts {
            if part.contains(&ser1){
                println!("{}", part);
                let mut dateData = String::from(part);
                let mut startDate = usize::from(dateData.find("BUILT_DATE: ").unwrap()) + 12;
                let endDate = 13;
                dateData.replace_range(..startDate,"");
                dateData.replace_range(13..,"");
                prevresults.push_str(part);
                let mut date = dateData.parse::<i64>().unwrap();
                date = date/86400000+25569;
                let dt = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap() + Duration::days(date);
                println!("Built Date: {}", dt);
            }
        }
        //DWE shenanigans
        for part in pieces {
            if part.contains(&ser1){
                println!("{}", part);
                prevresults.push_str(part);
            }
        }   
        //Zendesk shenanigans
        for part in bits {
            if part.contains(&ser1){
                println!("{}", part);
                prevresults.push_str(part);
            }
        } 
    }         
        ser1 = "".to_string();
        println!("Please enter search criteria:");
    }        
}