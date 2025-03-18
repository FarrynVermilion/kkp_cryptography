use std::{char, env, process::exit};

fn main() {
    fn take_input(debugging: bool) -> Vec<String> {
        //  input in rerminal example: cargo run "12345678123456781234567812345678" "You're in debugging tool with no input"
        // first arg is the key the rest is calue for encryption with the minimum of 2 args
        let mut result = Vec::new();
        let args: Vec<_> = env::args().collect();
        if args.len() > 1 {
            for i in args.iter().skip(1) {
                result.push(i.to_string());
            }
        }

        if debugging==true{
            if result.len() == 0 {
                result.extend(vec![
                    "12345678123456781234567812345678".to_string(),
                    "You're in debugging tool with no input".to_string(),
                    "This is a test, to not use the default value please enter a 32 char key and a value".to_string()
                ]);
            }
        }

        if result.len() == 1{
            println!("Theres only 1 value");
            exit(0);
        }

        if result.len() == 0 {
            println!("No input found");
            exit(0)
        }

        if result.get(0).unwrap().len() != 32 {
            println!("Input for key must be 32 characters long");
            exit(0)
        }

        result
    }

    fn convert_input_value_to_bytes(debugging: bool, values: Vec<String>) -> Vec<Vec<u8>> {
        let mut result:Vec<Vec<u8>>  = Vec::new();
        for value in values {
            let mut bytes = value.as_bytes().to_vec();
            let size = if bytes.len()%16 == 0 {0} else {16 - (value.len() % 16)};
            bytes.resize(bytes.len() + size, 0);
            
            if debugging == true {
                println!("\nValue: {}", value);
                println!("Length: {}", bytes.len());
                println!("size added: {}", size);
                for (index,byte) in bytes.iter().enumerate() {
                    println!("index: {index} \tByte: {byte:?} \thex: {byte:x} \tbit: {byte:08b} \tascii: {ch}",ch=*byte as char);
                }
            }

            result.push(bytes);
        }
        result
    }

    fn split_byte_array_to_an_array_of_4x4_matrix(debugging: bool, bytes_array: Vec<u8>) -> Vec<[[u8;4]; 4]> {
        let mut result=Vec::new();
        let array_matrix = bytes_array.chunks(16).collect::<Vec<_>>();
        for matrix in array_matrix {
            let mut matrix_2d = [[0;4]; 4];
            for i in 0..4 {
                for j in 0..4 {
                    matrix_2d[j][i] = matrix[i*4 + j];
                }
            }
            result.push(matrix_2d);
        }
        if debugging == true {
            for (index,matrix) in result.iter().copied().enumerate() {
                println!("\nmatrix ke : {}",index+1);
                for i in 0..4 {
                    println!("{:?}",matrix[i]);
                }
            }
        }

        result
    }

    fn key_expansion(debugging: bool){

    }
    fn add_round_key(debugging: bool){

    }
    fn substitution_box(debugging: bool){

    }
    fn shift_rows(debugging: bool){

    }
    fn mix_columns(debugging: bool){

    }

    // This is the main program that executes process
    // change value for debugging
    // this is default value
    let debugging = true;

    // take input
    let bytes_array = convert_input_value_to_bytes(debugging,take_input(debugging));

    // split key with its plain text array
    let key = bytes_array.get(0).unwrap();
    let data_array = bytes_array.iter().skip(1).collect::<Vec<_>>();

    // iterate over each arg
    for data in data_array {
        //arg will be split to an array of 4x4 matrix
        let matrix = split_byte_array_to_an_array_of_4x4_matrix(debugging,data.to_vec());
    }
}