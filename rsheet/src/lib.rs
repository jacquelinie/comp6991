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
use std::sync::{Arc, Mutex};

use std::thread;

type CellMap = Mutex<HashMap<String, CellValue>>;
type ExprMap = Mutex<HashMap<String, String>>;
type DepMap = Mutex<HashMap<String, HashSet<String>>>;

// Initialize the all global maps
lazy_static::lazy_static! {
    static ref CELL_MAP: CellMap = Mutex::new(HashMap::new());
    static ref EXPR_MAP: ExprMap = Mutex::new(HashMap::new());
    static ref DEPENDENCIES: DepMap = Mutex::new(HashMap::new());
    static ref DEPENDERS: DepMap = Mutex::new(HashMap::new());
    static ref CELL_ERRORS: ExprMap = Mutex::new(HashMap::new());
}

// ===================== STAGE 3 ============================

// SenderHandle to manage each sender's commands sequentially
struct SenderHandle {
    order_mutex: Mutex<()>, // Ensures sequential processing for a sender
}

/// Starts the server and listens for incoming connections. Handles `Get` and `Set` commands.
///
/// The function listens for incoming connections. Every new connection starts a new thread.
///
/// # Parameters
/// * `manager`: An instance of `Manager` that accepts new connections.
///
/// # Returns:
/// * Result
pub fn start_server<M>(mut manager: M) -> Result<(), Box<dyn Error>>
where
    M: Manager,
{
    let mut sender_map: HashMap<String, Arc<SenderHandle>> = HashMap::new();

    loop {
        info!("Just got a message!");
        match manager.accept_new_connection() {
            Connection::NewConnection {
                mut reader,
                mut writer,
            } => {
                let sender_name = reader.id();
                let sender_handle = sender_map.entry(sender_name.clone()).or_insert_with(|| {
                    Arc::new(SenderHandle {
                        order_mutex: Mutex::new(()),
                    })
                });

                let sender_handle = Arc::clone(sender_handle);

                // Spawn a thread to handle the new connection
                let handle = thread::spawn(move || {
                    if let Err(e) = handle_connection(sender_handle, &mut reader, &mut writer) {
                        eprintln!("Error handling connection for {}: {}", sender_name, e);
                    }
                });

                handle.join().unwrap();
            }
            Connection::NoMoreConnections => break,
        }
    }

    Ok(())
}

/// Function to handle each connection from a sender
/// The function listens for incoming commands from clients. When a `Get` command is received,
/// it retrieves the value of the requested cell. When a `Set` command is received, it processes
/// the expression, evaluates it, updates the cell, and resolves dependencies.
///
/// # Parameters
/// * `reader`: An Arc sender handle
/// * `recv`: A reader to receive commands
/// * `send`: A writer to send commands
///
/// # Returns
/// * Result
fn handle_connection(
    reader: Arc<SenderHandle>,
    recv: &mut dyn Reader,
    send: &mut dyn Writer,
) -> Result<(), Box<dyn Error>> {
    loop {
        match recv.read_message() {
            ReadMessageResult::Message(msg) => {
                // Acquire the lock for the sender's sequence
                let _lock = reader.order_mutex.lock().unwrap();

                // Handle the message
                let reply = match msg.parse::<Command>() {
                    Ok(command) => match command {
                        Command::Get { cell_identifier } => handle_get(&cell_identifier),
                        Command::Set {
                            cell_identifier,
                            cell_expr,
                        } => {
                            if let Some(reply) = handle_set(&cell_identifier, &cell_expr) {
                                reply
                            } else {
                                continue;
                            }
                        }
                    },
                    Err(e) => Reply::Error(format!("Error parsing command: {}", e)),
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

/// Converts a `CellIdentifier` into a `String` representation.
///
/// This function converts the row and column of the `CellIdentifier` into a cell address as a
/// string, such as "A1", where "A" is the column name and "1" is the row number.
///
/// # Parameters
/// * `cell_identifier`: A reference to a `CellIdentifier` that contains the row and column of a cell.
///
/// # Returns
fn cell_to_string(cell_identifier: &CellIdentifier) -> String {
    let col_name = column_number_to_name(cell_identifier.col);
    let row = cell_identifier.row + 1;

    // Return column and row
    format!("{}{}", col_name, row)
}

/// Evaluates a cell expression and stores the result in the `cells` map. Also processes any dependencies.
///
/// The expression is parsed and evaluated. If successful, the result is stored in the `cells` map.
/// If there is an error during evaluation, the error is stored in `cell_errors`. Additionally,
/// the dependencies of the cell are updated.
///
/// # Parameters
/// * `expr`: The `CellExpr` to evaluate.
/// * `cells`: A mutable reference to the map of cell values.
/// * `cell_address`: The address of the cell being evaluated.
/// * `cell_errors`: A mutable reference to the map of errors.
/// * `exprs`: A mutable reference to the map of expressions.
/// * `dependers`: A mutable reference to the map of dependers.
/// * `dependencies`: A mutable reference to the map of dependencies.
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

    let result: Result<CellValue, CellExprEvalError> = expr.evaluate(&variables);
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

/// Handles a `Get` command and retrieves the value of a cell.
///
/// The function checks if the requested cell exists and if it has any errors. If the cell has errors,
/// an error message is returned. Otherwise, the value of the cell is returned.
///
/// # Parameters
/// * `cell_identifier`: A reference to the `CellIdentifier` that identifies the cell.
///
/// # Returns
/// A `Reply` containing either the value of the cell or an error message.
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

/// Handles a `Set` command and evaluates an expression to update a cell's value.
///
/// The expression is parsed and evaluated, and the result is stored in the `cells` map. Dependencies are
/// updated as necessary. If the `Set` command cannot be processed, `None` is returned.
///
/// # Parameters
/// * `cell_identifier`: A reference to the `CellIdentifier` identifying the cell to update.
/// * `cell_expr`: The expression to evaluate for the cell.
///
/// # Returns
/// An `Option<Reply>` that is `None` when the operation is successful, or a `Reply` with an error if something goes wrong.
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

/// Parses the arguments in a cell expression and returns a map of variables to `CellArgument` values.
///
/// This function processes the variables in a cell expression, looking up their values in the `cells` map.
/// It handles matrix and vector expressions, resolving the coordinates and determining the appropriate value type.
///
/// # Parameters
/// * `cell_expr`: The expression containing variables.
/// * `cells`: A reference to the map of cell values.
/// * `new_dependers`: A mutable set that tracks the variables that depend on the evaluated expression.
///
/// # Returns
/// A `HashMap<String, CellArgument>` that maps each variable name to its corresponding `CellArgument` value.
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

/// Retrieves the value of a specific cell from the `cells` map.
///
/// This function looks up the value of a cell using its identifier and returns the value. If the cell
/// does not exist, `CellValue::None` is returned.
///
/// # Parameters
/// * `var`: The name of the variable representing the cell.
///
/// # Returns
/// The value of the cell, or `CellValue::None` if the cell does not exist.
fn get_value(var: &str, cells: &HashMap<String, CellValue>) -> CellValue {
    cells.get(var).cloned().unwrap_or(CellValue::None)
}

/// Retrieves the values of a vector of cells, either a row or column vector.
///
/// This function iterates over the coordinates of the vector, retrieves the corresponding values from
/// the `cells` map, and returns them as a `CellArgument::Vector` value.
///
/// # Parameters
/// * `coords`: A vector of strings representing the coordinates of the vector's cells.
/// * `cells`: A reference to the map of cell values.
/// * `dependers`: A mutable set that tracks which cells depend on this vector.
///
/// # Returns
/// A `CellArgument::Vector` containing the values of the vector's cells.
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

/// Retrieves the values of a matrix of cells, iterating over both rows and columns.
///
/// This function processes a matrix expression, retrieves the values of all the cells in the matrix,
/// and returns them as a `CellArgument::Matrix` value.
///
/// # Parameters
/// * `coords`: A vector of strings representing the coordinates of the matrix' cells.
/// * `cells`: A reference to the map of cell values.
/// * `dependers`: A mutable set that tracks which cells depend on this matrix.
///
/// # Returns
/// A `CellArgument::Matrix` containing the values of the matrix' cells.
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

/// Updates the dependencies of a cell when its value changes.
///
/// This function ensures that the dependencies of a cell are updated whenever its value changes.
/// It evaluates the affected expressions and updates the `dependencies` and `dependers` maps accordingly.
///
/// # Parameters
/// * `cell_address`: The address of the cell whose dependencies are being updated.
/// * `dependencies`: A mutable reference to the map of dependencies.
/// * `exprs`: A mutable reference to the map of expressions.
/// * `cells`: A mutable reference to the map of cell values.
/// * `cell_errors`: A mutable reference to the map of errors.
/// * `dependers`: A mutable reference to the map of dependers.
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

/// Processes the dependencies of a cell and updates the `dependers` and `dependencies` maps.
///
/// This function tracks the new and removed dependers, updating the dependencies accordingly. It also
/// triggers a re-evaluation of the affected cells to ensure their values are up-to-date.
///
/// # Parameters
/// * `new_dependers`: The new dependers of the cell.
/// * `cell_address`: The address of the cell whose dependencies are being processed.
/// * `exprs`: A mutable reference to the map of expressions.
/// * `cells`: A mutable reference to the map of cell values.
/// * `cell_errors`: A mutable reference to the map of errors.
/// * `dependers`: A mutable reference to the map of dependers.
/// * `dependencies`: A mutable reference to the map of dependencies.
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

    // 1. Test `cell_to_string` function with a sample CellIdentifier
    #[test]
    fn test_cell_to_string() {
        let cell_identifier = CellIdentifier { row: 0, col: 1 }; // Expected "B1"
        assert_eq!(cell_to_string(&cell_identifier), "B1");
    }

    // 2. Test `handle_set` for a valid expression
    #[test]
    fn test_handle_set_valid_expr() {
        let cell_id = CellIdentifier { row: 0, col: 1 }; // Cell "B1"
        let cell_expr = "5";

        // Execute
        assert!(handle_set(&cell_id, cell_expr).is_none());

        // Check CELL_MAP for the updated value
        let cells = CELL_MAP.lock().unwrap();
        let cell_address = cell_to_string(&cell_id);
        assert_eq!(cells.get(&cell_address), Some(&CellValue::Int(5)));
    }

    // 3. Test `evaluate_expr` for simple evaluation
    #[test]
    fn test_evaluate_expr() {
        let mut cells = HashMap::new();
        let mut exprs = HashMap::new();
        let mut cell_errors = HashMap::new();
        let mut dependers = HashMap::new();
        let mut dependencies = HashMap::new();

        let cell_address = "A1".to_string();
        let expr = CellExpr::new("10");

        evaluate_expr(
            expr,
            &mut cells,
            cell_address.clone(),
            &mut cell_errors,
            &mut exprs,
            &mut dependers,
            &mut dependencies,
        );

        assert_eq!(cells.get(&cell_address), Some(&CellValue::Int(10)));
        assert!(cell_errors.get(&cell_address).is_none());
    }

    // 4. Test `evaluate_expr` with a cell expression error
    #[test]
    fn test_evaluate_expr_with_error() {
        let mut cells = HashMap::new();
        let mut exprs = HashMap::new();
        let mut cell_errors = HashMap::new();
        let mut dependers = HashMap::new();
        let mut dependencies = HashMap::new();

        let cell_address = "A1".to_string();
        let cell_address_2 = "A2".to_string();

        let expr = CellExpr::new("invalid");
        let expr_2 = CellExpr::new("A1 + 1");

        evaluate_expr(
            expr,
            &mut cells,
            cell_address.clone(),
            &mut cell_errors,
            &mut exprs,
            &mut dependers,
            &mut dependencies,
        );
        evaluate_expr(
            expr_2,
            &mut cells,
            cell_address_2.clone(),
            &mut cell_errors,
            &mut exprs,
            &mut dependers,
            &mut dependencies,
        );

        assert!(cell_errors.contains_key(&cell_address_2));
    }

    // 5. Test `parse_expr_args` for argument parsing
    #[test]
    fn test_parse_expr_args() {
        let mut cells = HashMap::new();
        cells.insert("B1".to_string(), CellValue::Int(5));
        let expr = CellExpr::new("B1 + 10");
        let mut new_dependers = HashSet::new();

        let args = parse_expr_args(&expr, &cells, &mut new_dependers);
        assert!(args.contains_key("B1"));
        assert_eq!(args["B1"], CellArgument::Value(CellValue::Int(5)));
    }

    // 6. Test dependency management functions
    #[test]
    fn test_process_dependencies() {
        let mut cells = HashMap::new();
        let mut exprs = HashMap::new();
        let mut cell_errors = HashMap::new();
        let mut dependers = HashMap::new();
        let mut dependencies = HashMap::new();

        let cell_address = "A1".to_string();
        let new_dependers: HashSet<String> = vec!["B1".to_string()].into_iter().collect();

        process_dependencies(
            new_dependers.clone(),
            cell_address.clone(),
            &mut exprs,
            &mut cells,
            &mut cell_errors,
            &mut dependers,
            &mut dependencies,
        );

        assert!(dependers.contains_key(&cell_address));
        assert_eq!(dependers[&cell_address], new_dependers);
    }
}
