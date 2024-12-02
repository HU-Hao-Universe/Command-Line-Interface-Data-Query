use chrono::{Duration, NaiveDate};
use colored::*;
use env_logger;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::{self};

#[allow(non_snake_case)]

fn load_json<T>(filename: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de> + Default,
{
    let data = std::fs::read_to_string(filename)?;
    match serde_json::from_str(&data) {
        Ok(value) => Ok(value),
        Err(err) => {
            error!("Error deserializing JSON: {}", err);
            Ok(Default::default())
        }
    }
}

fn search_in_assemblies<'a>(
    assemblies: &'a AssembliesRoot,
    search_term: &str,
) -> Vec<&'a Assembly> {
    let search_term_upper = search_term.trim().to_uppercase();
    assemblies
        .asm
        .iter()
        .filter(|assembly| {
            assembly.serial_number.to_uppercase().contains(&search_term_upper)
                || assembly.sales_order.to_uppercase().contains(&search_term_upper)
                || assembly.description.to_uppercase().contains(&search_term_upper)
        })
        .collect()
}

fn search_in_drives<'a>(drives: &'a DrivesRoot, search_term: &str) -> Vec<&'a Drive> {
    let search_term_upper = search_term.trim().to_uppercase();
    drives
        .drive
        .iter()
        .filter(|drive| {
            drive.enclosure_sn.to_uppercase().contains(&search_term_upper)
                || drive.drive_sn.to_uppercase().contains(&search_term_upper)
                || drive.drive_manufacturer.to_uppercase().contains(&search_term_upper)
                || drive.model.to_uppercase().contains(&search_term_upper)
                || drive.part_number.to_uppercase().contains(&search_term_upper)
        })
        .collect()
}

fn search_in_zendesk<'a>(
    zendesk: &'a ZendeskRoot,
    search_term: &str,
) -> Vec<&'a ZendeskTicket> {
    let search_term_upper = search_term.trim().to_uppercase();
    zendesk
        .zendesk_ticket
        .iter()
        .filter(|ticket| {
            ticket.rma.to_string().to_uppercase().contains(&search_term_upper)
                || ticket.serial.to_uppercase().contains(&search_term_upper)
                || ticket.drive.to_uppercase().contains(&search_term_upper)
                || ticket.old_diagnosis.to_uppercase().contains(&search_term_upper)
                || ticket.new_diagnosis.to_uppercase().contains(&search_term_upper)
        })
        .collect()
}

fn print_assembly(
    assembly: &Assembly,
    indent: usize,
    built_by_colors: &HashMap<String, Color>,
) {
    let indent_str = "    ".repeat(indent + 1);
    println!("{}", "Assembly:".green().bold());

    if !assembly.serial_number.is_empty() {
        println!(
            "{}{} {}",
            indent_str,
            "Serial Number:".cyan(),
            assembly.serial_number.white()
        );
    }

    if assembly.built_date != 0 {
        print_build_date_with_warranty(assembly.built_date, indent + 1);
    }

    if !assembly.built_by.is_empty() {
        let built_by_color = built_by_colors
            .get(&assembly.built_by)
            .cloned()
            .unwrap_or(Color::White);
        println!(
            "{}{} {}",
            indent_str,
            "Built by:".cyan(),
            assembly.built_by.color(built_by_color)
        );
    }

    if !assembly.description.is_empty() {
        println!(
            "{}{} {}",
            indent_str,
            "Description:".cyan(),
            assembly.description.white()
        );
    }

    if !assembly.sales_order.is_empty() {
        println!(
            "{}{} {}",
            indent_str,
            "Sales Order:".cyan(),
            assembly.sales_order.white()
        );
    }
}

fn print_drive(drive: &Drive, indent: usize, manufacturer_colors: &HashMap<String, Color>) {
    let indent_str = "    ".repeat(indent);
    println!("{}", "Drive:".green().bold());

    if !drive.enclosure_sn.is_empty() {
        println!(
            "{}{} {}",
            indent_str,
            "Enclosure SN:".cyan(),
            drive.enclosure_sn.white()
        );
    }

    if !drive.drive_sn.is_empty() {
        println!(
            "{}{} {}",
            indent_str,
            "Drive SN:".cyan(),
            drive.drive_sn.white()
        );
    }

    if !drive.drive_manufacturer.is_empty() {
        let color = manufacturer_colors
            .get(&drive.drive_manufacturer)
            .cloned()
            .unwrap_or(Color::White);
        println!(
            "{}{} {}",
            indent_str,
            "Drive Manufacturer:".cyan(),
            drive.drive_manufacturer.color(color)
        );
    }

    if !drive.model.is_empty() {
        println!(
            "{}{} {}",
            indent_str,
            "Model:".cyan(),
            drive.model.white()
        );
    }

    if !drive.part_number.is_empty() {
        println!(
            "{}{} {}",
            indent_str,
            "Part Number:".cyan(),
            drive.part_number.white()
        );
    }
}

fn print_zendesk_ticket(ticket: &ZendeskTicket, indent: usize) {
    let indent_str = "    ".repeat(indent);
    println!("{}", "Zendesk Ticket:".green().bold());

    if ticket.rma != 0 {
        println!(
            "{}{} {}",
            indent_str,
            "RMA:".cyan(),
            ticket.rma.to_string().white()
        );
    }

    if !ticket.serial.is_empty() {
        println!(
            "{}{} {}",
            indent_str,
            "Serial:".cyan(),
            ticket.serial.white()
        );
    }

    if !ticket.drive.is_empty() {
        println!(
            "{}{} {}",
            indent_str,
            "Drive:".cyan(),
            ticket.drive.white()
        );
    }

    if !ticket.old_diagnosis.is_empty() {
        println!(
            "{}{} {}",
            indent_str,
            "Old Diagnosis:".cyan(),
            ticket.old_diagnosis.white()
        );
    }

    if !ticket.new_diagnosis.is_empty() {
        println!(
            "{}{} {}",
            indent_str,
            "New Diagnosis:".cyan(),
            ticket.new_diagnosis.white()
        );
    }
}

fn print_build_date_with_warranty(built_date: i64, indent: usize) {
    let indent_str = "    ".repeat(indent);
    let days_since_epoch = built_date / 86400000; // Assuming built_date is in milliseconds
    let dt = NaiveDate::from_ymd(1970, 1, 1) + Duration::days(days_since_epoch);
    println!(
        "{}{} {}",
        indent_str,
        "Built Date:".cyan(),
        dt.to_string().white()
    );

    // Check if the build date is within 3 years
    let current_date = chrono::Utc::now().naive_utc().date();
    let warranty_period = Duration::days(365 * 3);
    if current_date.signed_duration_since(dt) <= warranty_period {
        println!(
            "{}{}",
            indent_str,
            "Drive is under warranty".green().bold()
        );
    } else {
        println!(
            "{}{}",
            indent_str,
            "Drive is out of warranty".red().bold()
        );
    }
}

fn parse_and_print_date(date_str: &str) {
    if let Ok(mut date) = date_str.parse::<i64>() {
        date = date / 86400000 + 25569;
        let dt = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap() + Duration::days(date);
        println!("{}: {}", "Parsed Date".cyan(), dt.to_string().white());
    } else {
        println!("{}", "Invalid date format".red());
    }
}

// Assemblies JSON
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssembliesRoot {
    #[serde(rename = "ASM")]
    pub asm: Vec<Assembly>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assembly {
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

// Drive JSON
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrivesRoot {
    #[serde(rename = "DWE")]
    pub drive: Vec<Drive>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drive {
    #[serde(rename = "Enclosure SN")]
    pub enclosure_sn: String,
    #[serde(rename = "Drive SN")]
    pub drive_sn: String,
    #[serde(rename = "Drive Manufacturer")]
    pub drive_manufacturer: String,
    #[serde(rename = "Model")]
    pub model: String,
    #[serde(rename = "Part Number")]
    pub part_number: String,
}

// Zendesk JSON
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZendeskRoot {
    #[serde(rename = "ZEN")]
    pub zendesk_ticket: Vec<ZendeskTicket>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZendeskTicket {
    #[serde(rename = "RMA")]
    pub rma: i64,
    #[serde(rename = "Serial")]
    pub serial: String,
    #[serde(rename = "Drive")]
    pub drive: String,
    #[serde(rename = "OldDiagnosis")]
    pub old_diagnosis: String,
    #[serde(rename = "NewDiagnosis")]
    pub new_diagnosis: String,
}

fn assign_colors(names: &[&String]) -> HashMap<String, Color> {
    let color_list = vec![
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::BrightRed,
        Color::BrightGreen,
        Color::BrightYellow,
        Color::BrightBlue,
        Color::BrightMagenta,
        Color::BrightCyan,
    ];

    let mut colors_map = HashMap::new();
    for (i, name) in names.iter().enumerate() {
        let color = color_list[i % color_list.len()];
        colors_map.insert((*name).clone(), color);
    }
    colors_map
}

fn assign_pastel_colors(names: &[&String]) -> HashMap<String, Color> {
    // Define some pastel colors using RGB
    let pastel_color_values = vec![
        Color::TrueColor {
            r: 255,
            g: 179,
            b: 186,
        }, // Pastel Pink
        Color::TrueColor {
            r: 255,
            g: 223,
            b: 186,
        }, // Pastel Peach
        Color::TrueColor {
            r: 255,
            g: 255,
            b: 186,
        }, // Pastel Yellow
        Color::TrueColor {
            r: 186,
            g: 255,
            b: 201,
        }, // Pastel Green
        Color::TrueColor {
            r: 186,
            g: 225,
            b: 255,
        }, // Pastel Blue
        Color::TrueColor {
            r: 201,
            g: 186,
            b: 255,
        }, // Pastel Purple
    ];

    let mut colors_map = HashMap::new();
    for (i, name) in names.iter().enumerate() {
        let color = pastel_color_values[i % pastel_color_values.len()];
        colors_map.insert((*name).clone(), color);
    }
    colors_map
}

fn get_parent_manufacturer_name(
    manufacturer_name: &str,
    parent_names: &HashSet<String>,
) -> String {
    for parent in parent_names {
        if manufacturer_name != parent && manufacturer_name.starts_with(parent) {
            return parent.clone();
        }
    }
    manufacturer_name.to_string()
}
fn main() {
    env_logger::init();
    info!("Glyph Database Started. Type Q to quit.");
    println!(
        "{}",
        "Glyph Database Started. Type Q to quit.".green().bold()
    );

    info!("Loading Assemblies Database...");
    println!("{}", "Loading Assemblies Database...".yellow());
    let assemblers: AssembliesRoot = load_json("ASM.json").unwrap();
    info!("Assemblies Database Loaded.");
    println!("{}", "Assemblies Database Loaded.".green());

    info!("Loading Drives with Enclosures Database...");
    println!("{}", "Loading Drives with Enclosures Database...".yellow());
    let drive: DrivesRoot = load_json("DWE.json").unwrap();
    info!("Drives with Enclosures Database Loaded.");
    println!("{}", "Drives with Enclosures Database Loaded.".green());

    info!("Loading Zendesk Database...");
    println!("{}", "Loading Zendesk Database...".yellow());
    let zendesk_ticket: ZendeskRoot = load_json("ZEN.json").unwrap();
    info!("Zendesk Database Loaded.");
    println!("{}", "Zendesk Database Loaded.".green());

    // Count drive manufacturers
    let mut manufacturer_counts: HashMap<String, usize> = HashMap::new();
    for drive in &drive.drive {
        let manufacturer = drive.drive_manufacturer.trim().to_string();
        *manufacturer_counts.entry(manufacturer).or_insert(0) += 1;
    }

    // Convert HashMap to Vec<(String, usize)> and sort by counts descending
    let mut manufacturer_counts_vec: Vec<(String, usize)> =
        manufacturer_counts.into_iter().collect();
    manufacturer_counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // Take top 15
    let top_manufacturers = &manufacturer_counts_vec[..15.min(manufacturer_counts_vec.len())];

    // Extract parent manufacturer names (top 10 for example)
    let parent_names_set: HashSet<String> = manufacturer_counts_vec
        .iter()
        .take(10)
        .map(|(k, _)| k.clone())
        .collect();

    // Build a mapping from manufacturer names to parent names
    let mut manufacturer_to_parent: HashMap<String, String> = HashMap::new();
    for (manufacturer, _) in &manufacturer_counts_vec {
        let parent = get_parent_manufacturer_name(&manufacturer, &parent_names_set);
        manufacturer_to_parent.insert(manufacturer.clone(), parent);
    }

    // Assign colors to parent manufacturer names
    let parent_names_vec: Vec<&String> = parent_names_set.iter().collect();
    let parent_colors = assign_colors(&parent_names_vec);

    // Count built_by
    let mut built_by_counts: HashMap<String, usize> = HashMap::new();
    for assembly in &assemblers.asm {
        let built_by = assembly.built_by.trim().to_string();
        *built_by_counts.entry(built_by).or_insert(0) += 1;
    }

    // Convert HashMap to Vec<(String, usize)> and sort by counts descending
    let mut built_by_counts_vec: Vec<(String, usize)> =
        built_by_counts.into_iter().collect();
    built_by_counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // Take top 15
    let top_built_bys = &built_by_counts_vec[..15.min(built_by_counts_vec.len())];

    // Extract the names
    let built_bys: Vec<&String> = top_built_bys.iter().map(|(k, _)| k).collect();

    // Assign pastel colors to top built_bys
    let built_by_colors = assign_pastel_colors(&built_bys);

    // Count unique Enclosure S/N's
    let unique_enclosure_sns: HashSet<String> =
        drive.drive.iter().map(|d| d.enclosure_sn.clone()).collect();

    // Display counts of unique items
    println!("\n{}", "Counts of unique items:".cyan().bold());
    println!(
        "Unique Drive Manufacturers: {}",
        manufacturer_counts_vec.len()
    );
    println!("Unique Builders: {}", built_by_counts_vec.len());
    println!("Unique Enclosure S/N's: {}", unique_enclosure_sns.len());

    // Display top 15 Manufacturers and Builders side by side
    println!(
        "\n{}",
        "Top 15 Manufacturers and Builders:".cyan().bold()
    );
    let max_len = std::cmp::max(top_manufacturers.len(), top_built_bys.len());
    for i in 0..max_len {
        // Manufacturer
        let manufacturer_str = if i < top_manufacturers.len() {
            let (manufacturer, count) = &top_manufacturers[i];
            let parent = manufacturer_to_parent.get(manufacturer).unwrap();
            let color = parent_colors
                .get(parent)
                .cloned()
                .unwrap_or(Color::White);
            format!("{} ({})", manufacturer.color(color), count)
        } else {
            String::new()
        };

        // Builder
        let builder_str = if i < top_built_bys.len() {
            let (built_by, count) = &top_built_bys[i];
            let color = built_by_colors
                .get(built_by)
                .cloned()
                .unwrap_or(Color::White);
            format!("{} ({})", built_by.color(color), count)
        } else {
            String::new()
        };

        // Print them side by side
        println!("{:<50} {}", manufacturer_str, builder_str);
    }

    println!(
        "{}",
        "---------------------------------------------------------------------------------------------------------\n\n"
            .blue()
    );

    println!("{}", "Please enter search criteria:".cyan().bold());
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        debug!("User input: {}", input);

        if input.eq_ignore_ascii_case("Q") {
            info!("Exiting Glyph Database. Goodbye!");
            println!("{}", "Exiting Glyph Database. Goodbye!".green().bold());
            break;
        }

        if input.starts_with("#") {
            println!("{}", "Previous results functionality is not implemented in the new version.".yellow());
        } else if input.starts_with("$") {
            parse_and_print_date(&input[1..]);
        } else {
            info!("Searching all databases for: {}", input);
            let mut found = false;

            let mut printed_assemblies = HashSet::new();
            let mut printed_drives = HashSet::new();
            let mut printed_tickets = HashSet::new();

            let assembly_results = search_in_assemblies(&assemblers, input);
            if assembly_results.len() > 25 {
                println!(
                    "{}",
                    format!(
                        "Too many results found in assemblies ({}). Please refine your search.",
                        assembly_results.len()
                    )
                    .red()
                    .bold()
                );
                continue;
            }

            let drive_results = search_in_drives(&drive, input);
            if drive_results.len() > 25 {
                println!(
                    "{}",
                    format!(
                        "Too many results found in drives ({}). Please refine your search.",
                        drive_results.len()
                    )
                    .red()
                    .bold()
                );
                continue;
            }

            let zendesk_results = search_in_zendesk(&zendesk_ticket, input);
            if zendesk_results.len() > 25 {
                println!(
                    "{}",
                    format!(
                        "Too many results found in Zendesk tickets ({}). Please refine your search.",
                        zendesk_results.len()
                    )
                    .red()
                    .bold()
                );
                continue;
            }

            // Now process the results if none exceed 25
            if !assembly_results.is_empty() {
                found = true;
                for assembly in assembly_results {
                    if !printed_assemblies.contains(&assembly.serial_number) {
                        print_assembly(assembly, 0, &built_by_colors);
                        printed_assemblies.insert(assembly.serial_number.clone());

                        // Find related drives
                        let related_drives: Vec<&Drive> = drive
                            .drive
                            .iter()
                            .filter(|d| d.enclosure_sn == assembly.serial_number)
                            .collect();

                        for drive in related_drives {
                            if !printed_drives.contains(&drive.drive_sn) {
                                print_drive(drive, 1, &parent_colors);
                                printed_drives.insert(drive.drive_sn.clone());
                            }
                        }

                        // Find related tickets
                        let related_tickets: Vec<&ZendeskTicket> = zendesk_ticket
                            .zendesk_ticket
                            .iter()
                            .filter(|t| t.serial == assembly.serial_number)
                            .collect();

                        for ticket in related_tickets {
                            if !printed_tickets.contains(&ticket.rma) {
                                print_zendesk_ticket(ticket, 1); // Adjusted indentation
                                printed_tickets.insert(ticket.rma);
                            }
                        }
                    }
                }
            }

            // Now process drives that haven't been printed yet
            if !drive_results.is_empty() {
                found = true;
                for drive in drive_results {
                    if !printed_drives.contains(&drive.drive_sn) {
                        print_drive(drive, 0, &parent_colors);
                        printed_drives.insert(drive.drive_sn.clone());

                        // Find related tickets
                        let related_tickets: Vec<&ZendeskTicket> = zendesk_ticket
                            .zendesk_ticket
                            .iter()
                            .filter(|t| t.drive == drive.drive_sn)
                            .collect();

                        for ticket in related_tickets {
                            if !printed_tickets.contains(&ticket.rma) {
                                print_zendesk_ticket(ticket, 1); // Adjusted indentation
                                printed_tickets.insert(ticket.rma);
                            }
                        }
                    }
                }
            }

            // Now process Zendesk tickets that haven't been printed yet
            if !zendesk_results.is_empty() {
                found = true;
                for ticket in zendesk_results {
                    if !printed_tickets.contains(&ticket.rma) {
                        print_zendesk_ticket(ticket, 0);
                        printed_tickets.insert(ticket.rma);
                    }
                }
            }

            if !found {
                println!("{}", "No matching results found.".red().bold());
            }
        }

        println!("\n{}", "Please enter search criteria:".cyan().bold());
    }
}
