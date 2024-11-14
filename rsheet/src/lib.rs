use rsheet_lib::command::{CellIdentifier, Command};
use rsheet_lib::connect::{
    Connection, Manager, ReadMessageResult, Reader, WriteMessageResult, Writer,
};
use rsheet_lib::replies::Reply;
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::cell_expr::{CellArgument, CellExpr, CellExprEvalError};
use rsheet_lib::cells::{column_number_to_name, column_name_to_number};
use std::error::Error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use log::info;
use std::process;
use lazy_static::lazy_static;


type CellMap = Mutex<HashMap<String, CellValue>>;
type ExprMap = Mutex<HashMap<String, Arc<CellExpr>>>;

lazy_static! {
    static ref CELL_MAP: CellMap = Mutex::new(HashMap::new());
    static ref EXPR_MAP: ExprMap = Mutex::new(HashMap::new());
}

pub fn start_server<M>(mut manager: M) -> Result<(), Box<dyn Error>>
where
    M: Manager,
{
    let (mut recv, mut send) = match manager.accept_new_connection() {
        Connection::NewConnection { reader, writer } => (reader, writer),
        Connection::NoMoreConnections => {
            return Ok(());
        }
    };

    loop {
        info!("Just got message");
        match recv.read_message() {
            ReadMessageResult::Message(msg) => {
                let reply = match msg.parse::<Command>() {
                    Ok(command) => match command {
                        Command::Get { cell_identifier } => handle_get(&cell_identifier),
                        Command::Set { cell_identifier, cell_expr } => {
                            // Check for None
                            if let Some(reply) = handle_set(&cell_identifier, &cell_expr) {
                                reply
                            } else {
                                continue;
                            }
                        }
                    },
                    Err(e) => Reply::Error(e),
                };

                match send.write_message(reply) {
                    WriteMessageResult::Ok => {},
                    WriteMessageResult::ConnectionClosed => break,
                    WriteMessageResult::Err(e) => return Err(Box::new(e)),
                }
            }
            ReadMessageResult::ConnectionClosed => break,
            ReadMessageResult::Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(())
}

// ===================== HELPERS ============================

// Converts hashmap arguments
fn convert_to_arguments(cells: &HashMap<String, CellValue>) -> HashMap<String, CellArgument> {
    cells.iter()
        .map(|(key, value)| (key.clone(), CellArgument::Value(value.clone())))
        .collect()
}


// Converts CellIdentifier into String
fn cell_to_string(cell_identifier: &CellIdentifier) -> String {
    let col_name = column_number_to_name(cell_identifier.col);
    let row = cell_identifier.row + 1;

    // Return column and row
    format!("{}{}", col_name, row)
}

// Create CellValue
fn new_cell_value(value: &str) -> CellValue {
    if value.is_empty() {
        // None
        CellValue::None
    } else if let Ok(int_value) = value.parse::<i64>() {
        // Integer
        CellValue::Int(int_value)
    } else if value.starts_with("\"") && value.ends_with("\"") {
        // String
        let string_value = value[1..value.len() - 1].to_string();
        CellValue::String(string_value)
    } else {
        // Error
        eprintln!("Error parsing: Invalid value");
        process::exit(1);
    }
}

// Function to parse the cell expression into the hashmap of cellvalues
fn parse_expr_args(cell_expr: &CellExpr, cells: &HashMap<String, CellValue>) -> HashMap<String, CellArgument> {
    // Check for args
    let vars = cell_expr.find_variable_names();
    if vars.is_empty() {
        return HashMap::new();
    }

    let mut results = HashMap::new();
    for var in vars {
        // Value
        let value = cells.get(&var).cloned().unwrap_or(CellValue::None);


        // Matrix
        // if var.contains("_") {

        // } else {

        // }

        // Vector
        results.insert(var, value);
    }
    convert_to_arguments(&results)
}

// ===================== STAGE 1 ============================

// Handles get request
fn handle_get(cell_identifier: &CellIdentifier) -> Reply {
    let cell_address = cell_to_string(cell_identifier);
    let cells = CELL_MAP.lock().unwrap();
    println!("Handling Get....");

    match cells.get(&cell_address) {
        Some(CellValue::Error(err)) => Reply::Error(err.clone()),
        Some(value) => Reply::Value(cell_address, value.clone()),
        None => Reply::Value(cell_address, CellValue::None),
    }
}


// Handles set request
fn handle_set(cell_identifier: &CellIdentifier, cell_expr: &str) ->  Option<Reply> {
    let cell_address = cell_to_string(cell_identifier);
    let expr = CellExpr::new(cell_expr);
    println!("Handling Set....");

    // Get cells
    let mut cells = CELL_MAP.lock().unwrap();
    let mut exprs = EXPR_MAP.lock().unwrap();
    let variables = parse_expr_args(&expr, &cells);

    let result: Result<CellValue, CellExprEvalError> = expr.evaluate(&variables);

    // Set result
    match result {
        Ok(value) => {
            cells.insert(cell_address.clone(), value.clone());
            exprs.insert(cell_address.clone(), expr);
            None
        }
        Err(e) => Some(Reply::Error(format!("Error with setting value: {:?}", e))),
    }
}

