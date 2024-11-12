use rsheet_lib::command::{CellIdentifier, Command};
use rsheet_lib::connect::{
    Connection, Manager, ReadMessageResult, Reader, WriteMessageResult, Writer,
};
use rsheet_lib::replies::Reply;
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::cell_expr::{CellArgument, CellExpr, CellExprEvalError};
use std::error::Error;
use std::collections::HashMap;
use std::sync::Mutex;
use log::info;
use lazy_static::lazy_static;


type CellMap = Mutex<HashMap<String, CellValue>>;

// Initialize the cell map globally or within the server instance
lazy_static::lazy_static! {
    static ref CELL_MAP: CellMap = Mutex::new(HashMap::new());
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


// Converts hashmap arguments
fn convert_to_arguments(cells: &HashMap<String, CellValue>) -> HashMap<String, CellArgument> {
    cells.iter()
        .map(|(key, value)| (key.clone(), CellArgument::Value(value.clone())))
        .collect()
}


// Converts CellIdentifier into String
fn cell_to_string(cell_identifier: &CellIdentifier) -> String {
    // Convert the `col` to letters
    let mut col = cell_identifier.col + 1;
    let mut col_name = String::new();
    let row = cell_identifier.row + 1;

    while col > 0 {
        col -= 1;
        col_name.insert(0, (b'A' + (col % 26) as u8) as char);
        col /= 26;
    }
    // Return column and row
    format!("{}{}", col_name, row)
}

// ===================== STAGE 1 ============================

// Handles get request
fn handle_get(cell_identifier: &CellIdentifier) -> Reply {
    let cell_address = cell_to_string(cell_identifier);
    let cells = CELL_MAP.lock().unwrap();
    match cells.get(&cell_address) {
        Some(CellValue::Error(err)) => Reply::Error(err.clone()),
        Some(value) => Reply::Value(cell_address, value.clone()),
        None => Reply::Value(cell_address, CellValue::None),
    }
}


// Handles set request
fn handle_set(cell_identifier: &CellIdentifier, cell_expr: &str) ->  Option<Reply> {
    let expr = CellExpr::new(cell_expr);
    let cell_address = cell_to_string(cell_identifier);

    // Acquire the lock and convert to the expected argument type
    let mut cells = CELL_MAP.lock().unwrap();
    let cell_arguments = convert_to_arguments(&*cells);

    // Check for expression
    let vars = expr.find_variable_names();
    let result: Result<CellValue, CellExprEvalError> = if vars.is_empty() {
        // If no variables, try parsing the cell expression directly
        parse_cell_value(cell_expr)
    } else {
        // Otherwise, evaluate the expression
        expr.evaluate(&cell_arguments)
    };

    // Set result
    match result {
        Ok(value) => {
            cells.insert(cell_address.clone(), value.clone());
            None
        }
        Err(e) => Some(Reply::Error(format!("Error with setting value: {:?}", e))),
    }
}

// Function to parse the cell expression into the correct CellValue type
fn parse_cell_value(cell_expr: &str) -> Result<CellValue, CellExprEvalError> {
    if cell_expr.is_empty() {
        // None
        Ok(CellValue::None)
    } else if let Ok(int_value) = cell_expr.parse::<i64>() {
        // Integer
        Ok(CellValue::Int(int_value))
    } else if cell_expr.starts_with("\"") && cell_expr.ends_with("\"") {
        // String
        let string_value = cell_expr[1..cell_expr.len() - 1].to_string();
        Ok(CellValue::String(string_value))
    } else {
        // Error
        Err(CellExprEvalError::VariableDependsOnError)
    }
}

