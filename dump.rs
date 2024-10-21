        // Check indentation for loops
        // let curr_indentation = count_indentation(line);
        // let mut if_indentation = 0;
        // let mut while_indentation = 0;
        // let inputs: Vec<&str> = line.split_whitespace().collect();
        // if inputs[0] == "IF" {
        //     if_indentation = count_indentation(line);

        // }
        // if inputs[0] == "WHILE" {
        //     while_indentation = count_indentation(line);
        // }
        // // If loop
        // pub fn if_loop(curr_loop: Vec<&str>, image: &mut Image,
        //     variables: &mut HashMap<String, String>, line_number: &i32) {
        //     println!("If Looping");
        //     for line in curr_loop[1..].iter() {
        //         let line_num = line_number + 1;
        //         if let Err(e) = execute_command(self, image, variables, line, &line_num) {
        //             eprintln!("Error: {}", e);
        //             process::exit(1);
        //         }
        //     }
        // }
        // ================ TASK 3 ================
        // "IF" => { // IF EQ v1 v2
        //     error_extra_arguments(&inputs, 4);
        //     let v1_str = arguments.get(0).ok_or(format!("Error on line {}: Empty line", line_number))?;
        //     let v2_str = arguments.get(1).ok_or(format!("Error on line {}: Empty line", line_number))?;
        //     let v1: i32 = v1_str.parse().map_err(|_| format!("Error on line {}: If statement requires a value.", line_number))?;
        //     let v2: i32 = v2_str.parse().map_err(|_| format!("Error on line {}: If statement requires a value.", line_number))?;

        //     if v1 == v2 {
        //         let mut curr_loop: Vec<&str> = Vec::new();
        //         while inputs[0] != "[" && curr_indentation != if_indentation {
        //             curr_loop.push(line);
        //         }
        //         turtle.if_loop(curr_loop, image, variables, line_number);
        //     }
        // }

        // "WHILE" => { // IF EQ v1 v2

        // }