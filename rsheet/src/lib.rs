use log::info;
use rsheet_lib::cell_expr::{CellArgument, CellExpr, CellExprEvalError};
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::cells::column_number_to_name;
use rsheet_lib::command::{CellIdentifier, Command};
use rsheet_lib::connect::{
    Connection, Manager, ReadMessageResult, Reader, WriteMessageResult, Writer,
};
use rsheet_lib::replies::Reply;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::sync::Mutex;
// use std::process;
// use lazy_static::lazy_static;

type CellMap = Mutex<HashMap<String, CellValue>>;
type ExprMap = Mutex<HashMap<String, String>>;
type DepMap = Mutex<HashMap<String, HashSet<String>>>;

// Initialize the cell map globally or within the server instance
lazy_static::lazy_static! {
    static ref CELL_MAP: CellMap = Mutex::new(HashMap::new());
    static ref EXPR_MAP: ExprMap = Mutex::new(HashMap::new());
    static ref DEPENDENCIES: DepMap = Mutex::new(HashMap::new());
    static ref DEPENDERS: DepMap = Mutex::new(HashMap::new());
    static ref CELL_ERRORS: ExprMap = Mutex::new(HashMap::new());
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
                        Command::Set {
                            cell_identifier,
                            cell_expr,
                        } => {
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
                    WriteMessageResult::Ok => {}
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

// Converts CellIdentifier into String
fn cell_to_string(cell_identifier: &CellIdentifier) -> String {
    let col_name = column_number_to_name(cell_identifier.col);
    let row = cell_identifier.row + 1;

    // Return column and row
    format!("{}{}", col_name, row)
}

// Evalutes expression
fn evaluate_expr(
    expr: CellExpr,
    cells: &mut HashMap<String, CellValue>,
    cell_address: String,
    cell_errors: &mut HashMap<String, String>,
    exprs: &mut HashMap<String, String>,
    dependers: &mut HashMap<String, HashSet<String>>,
    dependencies: &mut HashMap<String, HashSet<String>>,
) {
    let mut new_dependers: HashSet<String> = HashSet::new();
    let variables = parse_expr_args(&expr, cells, &mut new_dependers);

    // println!("Evaluating....");

    let result: Result<CellValue, CellExprEvalError> = expr.evaluate(&variables);
    // println!("Retrieved Result: {:?}", result);
    match result {
        // Ok -> Store
        Ok(value) => {
            cells.insert(cell_address.clone(), value.clone());
            cell_errors.remove(&cell_address);
        }
        // Eval Error
        Err(e) => {
            cell_errors.insert(cell_address.clone(), format!("{:?}", e));
        }
    }

    // println!("Finished Evaluating...: {}", cell_address);

    process_dependencies(
        new_dependers.clone(),
        cell_address.clone(),
        exprs,
        cells,
        cell_errors,
        dependers,
        dependencies,
    );
}

// ===================== STAGE 1 ============================

// Handles get request
fn handle_get(cell_identifier: &CellIdentifier) -> Reply {
    let cell_address = cell_to_string(cell_identifier);

    let cell_errors = CELL_ERRORS.lock().unwrap();
    let cells = CELL_MAP.lock().unwrap();

    // Check if any cells are depending on errors
    if let Some(error) = cell_errors.get(&cell_address) {
        return Reply::Error(format!(
            "Cannot get cell {}: it depends on an error - {}",
            cell_address, error
        ));
    }

    // Otherwise, proceed with checking cells
    match cells.get(&cell_address) {
        Some(value) => Reply::Value(cell_address, value.clone()),
        None => Reply::Value(cell_address, CellValue::None),
    }
}

// Handles set request
fn handle_set(cell_identifier: &CellIdentifier, cell_expr: &str) -> Option<Reply> {
    let mut cells = CELL_MAP.lock().unwrap();
    let mut exprs = EXPR_MAP.lock().unwrap();
    let mut cell_errors = CELL_ERRORS.lock().unwrap();
    let mut dependers = DEPENDERS.lock().unwrap();
    let mut dependencies = DEPENDENCIES.lock().unwrap();

    let cell_address = cell_to_string(cell_identifier);
    let expr = CellExpr::new(cell_expr);

    exprs.insert(cell_address.clone(), (*cell_expr).to_string());

    evaluate_expr(
        expr,
        &mut cells,
        cell_address.clone(),
        &mut cell_errors,
        &mut exprs,
        &mut dependers,
        &mut dependencies,
    );
    // update dependencies

    None
}

// ===================== STAGE 2 ============================

// Function to parse the cell expression into the hashmap of cellvalues
fn parse_expr_args(
    cell_expr: &CellExpr,
    cells: &HashMap<String, CellValue>,
    new_dependers: &mut HashSet<String>,
) -> HashMap<String, CellArgument> {
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
                    value = get_matrix(coords, cells, new_dependers);
                }
                // Vector: either rows or columns are the same
                else if row1 == row2 || col1 == col2 {
                    value = get_vector(coords, cells, new_dependers);
                }
            }
        } else {
            new_dependers.insert(var.clone());
            value = CellArgument::Value(get_value(&var, cells));
        }
        // Insert
        results.insert(var, value);
    }
    results
}

// Get value from cell
fn get_value(var: &str, cells: &HashMap<String, CellValue>) -> CellValue {
    cells.get(var).cloned().unwrap_or(CellValue::None)
}

// Get vector of cells
fn get_vector(
    coords: Vec<&str>,
    cells: &HashMap<String, CellValue>,
    dependers: &mut HashSet<String>,
) -> CellArgument {
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
            dependers.insert(coord.clone());
            vector_values.push(get_value(&coord, cells));
        }
    } else if start_col == end_col {
        // Column vector (iterate over rows in the same column)
        for row in start_row..=end_row {
            let coord = format!("{}{}", row, start_col);
            dependers.insert(coord.clone());
            vector_values.push(get_value(&coord, cells));
        }
    }

    CellArgument::Vector(vector_values)
}

// Get matrix of cells
fn get_matrix(
    coords: Vec<&str>,
    cells: &HashMap<String, CellValue>,
    dependers: &mut HashSet<String>,
) -> CellArgument {
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
            dependers.insert(coord.clone());
            row_values.push(get_value(&coord, cells));
        }
        matrix_values.push(row_values);
    }

    CellArgument::Matrix(matrix_values)
}

// ===================== STAGE 4 + 5 ============================

// Update dependencies of cell address based on new value
fn update_dependencies(
    cell_address: String,
    dependencies: &mut HashMap<String, HashSet<String>>,
    exprs: &mut HashMap<String, String>,
    cells: &mut HashMap<String, CellValue>,
    cell_errors: &mut HashMap<String, String>,
    dependers: &mut HashMap<String, HashSet<String>>,
) {
    let mut dependencies_clone = dependencies.clone();
    let deps = dependencies_clone.entry(cell_address.clone()).or_default();

    // Go through any dependencies and update
    for dep in deps.iter() {
        let expr_string = exprs.entry(dep.clone()).or_default();
        let expr = CellExpr::new(expr_string); // No need to insert
        evaluate_expr(
            expr,
            cells,
            dep.clone(),
            cell_errors,
            exprs,
            dependers,
            dependencies,
        ); // Should never have a circular case
    }
}

// Process all dependencies and check for new and remove old ones
fn process_dependencies(
    new_dependers: HashSet<String>,
    cell_address: String,
    exprs: &mut HashMap<String, String>,
    cells: &mut HashMap<String, CellValue>,
    cell_errors: &mut HashMap<String, String>,
    dependers: &mut HashMap<String, HashSet<String>>,
    dependencies: &mut HashMap<String, HashSet<String>>,
) {
    let old_dependers = dependers.remove(&cell_address).unwrap_or_default();

    let added_dependers = new_dependers.difference(&old_dependers);
    let removed_dependers = old_dependers.difference(&new_dependers);

    // Insert new dependers
    dependers.insert(cell_address.clone(), new_dependers.clone());

    // Edit old dependencies (for removed dependers)
    for dependency in removed_dependers {
        if let Some(mut curr_dependencies) = dependencies.remove(dependency) {
            curr_dependencies.remove(&cell_address);
            dependencies.insert(dependency.to_string(), curr_dependencies);
        }
    }

    // Add new dependencies (for added dependers)
    for dependency in added_dependers {
        // Use entry to get a mutable reference to the dependencies of the dependency
        let added_dependencies = dependencies.entry(dependency.to_string()).or_default();
        added_dependencies.insert(cell_address.clone());
    }

    update_dependencies(
        cell_address.clone(),
        dependencies,
        exprs,
        cells,
        cell_errors,
        dependers,
    );
}



// ===================== TESTS ============================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_cell_none() {
        let result = CellExpr::new("()").evaluate(&HashMap::new());
        assert_eq!(result, Ok(CellValue::None));
    }

    #[test]
    fn test_run_values_only() {
        let result = CellExpr::new("2 + 2").evaluate(&HashMap::new());
        assert_eq!(result, Ok(CellValue::Int(4)));
    }

    #[test]
    fn test_run_value() {
        let result = CellExpr::new("2").evaluate(&HashMap::new());
        assert_eq!(result, Ok(CellValue::Int(2)));
    }

    #[test]
    fn test_run_cell_vector() {
        let vector = CellArgument::Vector(vec![
            CellValue::Int(1),
            CellValue::Int(2),
            CellValue::Int(3),
        ]);
        let result =
            CellExpr::new("sum(A1_A3)").evaluate(&HashMap::from([("A1_A3".to_string(), vector)]));
        assert_eq!(result, Ok(CellValue::Int(6)));
    }

    #[test]
    fn test_run_cell_value() {
        let values = HashMap::from([
            ("A1".to_string(), CellArgument::Value(CellValue::Int(1))),
            ("A2".to_string(), CellArgument::Value(CellValue::Int(2))),
            ("A3".to_string(), CellArgument::Value(CellValue::Int(3))),
        ]);
        let result = CellExpr::new("A1 + A2 + A3").evaluate(&values);

        assert_eq!(
            CellExpr::new("A1 + A2 + A3").find_variable_names(),
            vec!["A1".to_string(), "A2".to_string(), "A3".to_string()]
        );
        assert_eq!(result, Ok(CellValue::Int(6)));
    }

    #[test]
    fn test_run_cell_matrix() {
        let matrix = CellArgument::Matrix(vec![
            vec![CellValue::Int(1), CellValue::Int(2)],
            vec![CellValue::Int(3), CellValue::Int(4)],
        ]);
        let result =
            CellExpr::new("sum(A1_B2)").evaluate(&HashMap::from([("A1_B2".to_string(), matrix)]));
        assert_eq!(result, Ok(CellValue::Int(10)));
    }

    #[test]
    fn test_run_cell_error() {
        let result = CellExpr::new("asdf").evaluate(&HashMap::new());
        assert!(matches!(result, Ok(CellValue::Error(_))));
    }

    #[test]
    fn test_depend_on_error() {
        let values = HashMap::from([
            ("A1".to_string(), CellArgument::Value(CellValue::Int(1))),
            (
                "A2".to_string(),
                CellArgument::Value(CellValue::Error("some existing error".to_string())),
            ),
        ]);
        let result = CellExpr::new("A1 + A2").evaluate(&values);
        assert!(matches!(
            result,
            Err(CellExprEvalError::VariableDependsOnError)
        ));
    }

    #[test]
    fn test_depend_on_error_vector() {
        let values = HashMap::from([
            ("A1".to_string(), CellArgument::Value(CellValue::Int(1))),
            (
                "A2_A3".to_string(),
                CellArgument::Vector(vec![
                    CellValue::Int(10),
                    CellValue::Error("some existing error".to_string()),
                ]),
            ),
        ]);
        let result = CellExpr::new("A1 + sum(A2_A3)").evaluate(&values);
        assert!(matches!(
            result,
            Err(CellExprEvalError::VariableDependsOnError)
        ));
    }

    #[test]
    fn test_depend_on_error_matrix() {
        let values = HashMap::from([
            ("A1".to_string(), CellArgument::Value(CellValue::Int(1))),
            (
                "A2_B3".to_string(),
                CellArgument::Matrix(vec![
                    vec![
                        CellValue::Int(10),
                        CellValue::Error("some existing error".to_string()),
                    ],
                    vec![CellValue::Int(20), CellValue::Int(50)],
                ]),
            ),
        ]);
        let result = CellExpr::new("A1 + sum(A2_B3)").evaluate(&values);
        assert!(matches!(
            result,
            Err(CellExprEvalError::VariableDependsOnError)
        ));
    }
}
