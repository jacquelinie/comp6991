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
use std::sync::Mutex;
use log::info;
use std::process;
// use lazy_static::lazy_static;


type CellMap = Mutex<HashMap<String, CellValue>>;
type ExprMap = Mutex<HashMap<String, CellExpr>>;

// Initialize the cell map globally or within the server instance
lazy_static::lazy_static! {
    static ref CELL_MAP: CellMap = Mutex::new(HashMap::new());
    static ref EXPR_MAP: CellMap = Mutex::new(HashMap::new());
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
        let mut value = CellArgument::Value(CellValue::None);

        // Matrix or Vector
        if var.contains("_") {
            let coords: Vec<&str> = var.split('_').collect();
            if coords.len() == 2 {
                // Extract rows and columns once
                let row1 = coords[0].chars().next();
                let row2 = coords[1].chars().next();
                let col1 = &coords[0][1..];
                let col2 = &coords[1][1..];

                // Matrix: both row and column are different
                if row1 != row2 && col1 != col2 {
                    value = get_matrix(coords, &cells);
                }
                // Vector: either rows or columns are the same
                else if row1 == row2 || col1 == col2 {
                    value = get_vector(coords, &cells);
                }
            }
        } else {
            value = CellArgument::Value(get_value(&var, &cells));
        }
        // Insert
        results.insert(var, value);
    }
    results
}

// Get value from cell
fn get_value(var: &str, cells: &HashMap<String, CellValue>) -> CellValue{
    cells.get(var).cloned().unwrap_or(CellValue::None)
}

// Get vector of cells
fn get_vector(coords: Vec<&str>, cells: &HashMap<String, CellValue>) -> CellArgument {
    // Extract start and end coordinates
    let start = coords[0];
    let end = coords[1];

    // Determine if we're dealing with a row or column vector
    let start_row = start.chars().next().unwrap();
    let end_row = end.chars().next().unwrap();
    let start_col: i32 = start[1..].parse().unwrap();
    let end_col: i32 = end[1..].parse().unwrap();

    let mut vector_values = Vec::new();
    if start_row == end_row {
        // Row vector (iterate over columns in the same row)
        for col in start_col..=end_col {
            let coord = format!("{}{}", start_row, col);
            vector_values.push(get_value(&coord, cells));
        }
    } else if start_col == end_col {
        // Column vector (iterate over rows in the same column)
        for row in start_row..=end_row {
            let coord = format!("{}{}", row, start_col);
            vector_values.push(get_value(&coord, cells));
        }
    }

    CellArgument::Vector(vector_values)
}

// Get matrix of cells
fn get_matrix(coords: Vec<&str>, cells: &HashMap<String, CellValue>) -> CellArgument {
    // Extract start and end coordinates
    let start = coords[0];
    let end = coords[1];

    let start_row = start.chars().next().unwrap();
    let end_row = end.chars().next().unwrap();
    let start_col: i32 = start[1..].parse().unwrap();
    let end_col: i32 = end[1..].parse().unwrap();

    let mut matrix_values = Vec::new();
    for row in start_row..=end_row {
        let mut row_values = Vec::new();
        for col in start_col..=end_col {
            let coord = format!("{}{}", row, col);
            row_values.push(get_value(&coord, cells));
        }
        matrix_values.push(row_values);
    }

    CellArgument::Matrix(matrix_values)
}

// ===================== STAGE 1 ============================

// Handles get request
fn handle_get(cell_identifier: &CellIdentifier) -> Reply {
    let cell_address = cell_to_string(cell_identifier);
    let cells = CELL_MAP.lock().unwrap();
    // println!("Handling Get....");

    match cells.get(&cell_address) {
        Some(value) => Reply::Value(cell_address, value.clone()),
        None => Reply::Value(cell_address, CellValue::None),
    }
}


// Handles set request
fn handle_set(cell_identifier: &CellIdentifier, cell_expr: &str) ->  Option<Reply> {
    let cell_address = cell_to_string(cell_identifier);
    let expr = CellExpr::new(cell_expr);
    // println!("Handling Set....");

    // Get cells
    let mut cells = CELL_MAP.lock().unwrap();
    let variables = parse_expr_args(&expr, &cells);

    let result: Result<CellValue, CellExprEvalError> = expr.evaluate(&variables);

    // Set result
    match result {
        Ok(value) => {
            cells.insert(cell_address.clone(), value.clone());
            None
        }
        Err(e) => Some(Reply::Error(format!("Error with setting value: {:?}", e))),
    }
}

