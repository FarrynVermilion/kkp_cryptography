use std::{char, env, process::exit};

fn main() {
    // take input fungsi buat ambil input dari terminal 
    // kalo not with value true gk perlu ambil input buat debugginh aja
    fn take_input(not_with_value: bool) -> Vec<String> {
        //  input in rerminal example: cargo run "12345678123456781234567812345678" "You're in debugging tool with no input"
        // first arg is the key the rest is value for encryption with the minimum of 2 args
        let mut result = Vec::new();
        let args: Vec<_> = env::args().collect();
        if args.len() > 1 {
            for i in args.iter().skip(1) {
                result.push(i.to_string());
            }
        }

        if not_with_value==true{
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

    // fungsi ubah input char ke byte
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

    // fungsi kelompokin bytes ke array 4x4
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
                println!("\nplain text matrix ke : {}",index+1);
                for i in 0..4 {
                    println!("{:?}",matrix[i]);
                }
            }
        }

        result
    }

    // fungsi utama buat key expansion
    fn key_expansion(debugging: bool, key: Vec<u8>)-> Vec<[[u8;4]; 4]> {
        let rcon: [u8;10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];

        let mut rkey: Vec<[[u8;4]; 4]>= Vec::new();

        for i in 0..8 {
            let mut matrix1 = [[0;4]; 4];
            let mut matrix2 = [[0;4]; 4];
            match i {
                // masukin key original
                0 => {
                    for y in 0..4 {
                        for x in 0..4 {
                            matrix1[x][y] = key[y*4 + x];
                            matrix2[x][y] = key[y*4 + x+16];
                        }
                    }
                },
                // proses key expansion utama
                1..8 => {
                    // buat prev matrix 8x4 nuat mempermudah penghitungan
                    let mut prev_matrix = [[0;8]; 4];
                    for x in 0..4 {
                        prev_matrix[x] = [&rkey[i*2-2][x][..],&rkey[i*2-1][x][..]].concat().try_into().unwrap();
                    }
                    // buat oprasi matrix per kolom beda beda
                    for y in 0..8 {
                        let mut xor_arr = [0;4];
                        // kolom 0 arr yang di xor kan perlu ada shift dan s box dan di xor
                        if y == 0 {
                            let sub_data = {
                                let mut sub_data:[u8;4]=[0;4];
                                let shifted =shift_columns(debugging, [prev_matrix[0][0],prev_matrix[1][0],prev_matrix[2][2],prev_matrix[3][0]]);
                                for x in 0..4 {
                                    sub_data[x]=substitution_box(debugging, shifted[x]);
                                }
                                sub_data
                            };
                            for x in 0..4 {
                                if x==0{
                                    xor_arr[x] = sub_data[x] ^ rcon[i-1];
                                    if debugging == true {
                                        println!("rcon\t: dec :{rc:?}\tbit:{rc:08b}", rc=rcon[i-1]);
                                    }
                                }else {
                                    xor_arr[x] = sub_data[x];
                                }
                                if debugging == true {
                                    println!("sub_data\t: dec:{sd:?}\tbit:{sd:08b}", sd=sub_data[x]);
                                }
                            }
                        // kolom 5 hanya s box
                        }else if y == 4 {
                            for x in 0..4 {
                                xor_arr[x] = substitution_box(debugging, prev_matrix[x][y-1]);
                            };
                        // kolom selain 1 dan 4
                        }else {
                            for x in 0..4 {
                                xor_arr[x] = prev_matrix[x][y-1];
                            }
                        }
                        // proses xor hasil di taro di prev matrix
                        for x in 0..4 {
                            if debugging == true {
                                println!("prev_matrix\t: dec:{p:?}\tbit:{p:08b}", p=prev_matrix[x][y]);
                            }
                            
                            prev_matrix[x][y] = xor_arr[x] ^ prev_matrix[x][y];
                            if debugging == true {
                                println!("xor_arr\t: {:08b}", xor_arr[x]);
                                println!("current\t: dec:{p:?}\tbit:{p:08b}", p=prev_matrix[x][y]);
                            }
                            
                        }
                    }
                    // pecah prev matrix jadi 2 4x4 matrix
                    for x in 0..4 {
                        let(a,b) = prev_matrix[x].split_at(4);
                        matrix1[x] = a.try_into().unwrap();
                        matrix2[x] = b.try_into().unwrap();
                    }
                    
                },
                _ => println!("Error"),
                
            }
            rkey.push(matrix1);
            rkey.push(matrix2);
            
        }
        // print rkey
        if debugging == true {
            println!("RCON: {rcon:?}");
            for (index,matrix) in rkey.iter().copied().enumerate() {
                println!("\nkey matrix ke : {index}");
                for i in 0..4 {
                    println!("{:?}",matrix[i]);
                }
            }
        }
        rkey
    }

    // shift kolom ke kiridan paling kiri ke kanan
    fn shift_columns(debugging: bool, word: [u8; 4])->[u8; 4] {
        let shifted= [word[1], word[2], word[3], word[0]];
        if debugging == true {
            println!("orgin\t:{word:?}");
            println!("altered\t:{shifted:?}");
        }
        shifted
    }
    // perkalian matrix xor round key dengan matrix
    fn add_round_key( matrix: [[u8;4]; 4], key: [[u8;4]; 4])->[[u8;4]; 4] {
        let mut result = [[0;4]; 4];
        for x in 0..4 {
            for y in 0..4 {
                result[x][y] = matrix[x][y] ^ key[x][y];
            }
        }
        result
    }
    // substitution box
    fn substitution_box(debugging: bool, data:u8)->u8 {

        let sbox: [[u8; 16]; 16] = [
            [0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76],
            [0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0],
            [0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15],
            [0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75],
            [0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84],
            [0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf],
            [0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8],
            [0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2],
            [0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73],
            [0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb],
            [0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79],
            [0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08],
            [0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a],
            [0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e],
            [0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf],
            [0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16]
        ];
        // kolom / col / x
        let mod16= (data as usize) % 16;
        // baris / row / y
        let base16= ((data as usize)-mod16)/16;
        // ambil data substitution box
        let sub_data=sbox[base16][mod16];
        if debugging==true{
            println!("orgin\t\t:hex:{data:x}\t: dec:{data:?}");
            println!("baris\t\t:{base16:x}");
            println!("kolom\t\t:{mod16:x}");
            println!("sub data\t:{sub_data:x}");
        }
        sub_data
        
    }
    // shift rows per baris
    fn shift_rows(matrix: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
        let mut result=[[0;4];4];
        for x in 0..4 {
            let (a,b)=matrix[x].split_at(x);
            let mut y = 0;
            for i in 0..b.len(){
                result[x][y]=b[i];
                y+=1;
            }
            for i in 0..a.len(){
                result[x][y]=a[i];
                y+=1;
            }
        }
        result
    }
    // prosose mix column
    fn mix_columns(debugging: bool,matrix:[[u8; 4]; 4]) -> [[u8; 4]; 4] {
        let mut result=[[0u8; 4]; 4];
        let matrix_multiplication: [[u8; 4]; 4] = [
            [0x02, 0x03, 0x01, 0x01],
            [0x01, 0x02, 0x03, 0x01],
            [0x01, 0x01, 0x02, 0x03],
            [0x03, 0x01, 0x01, 0x02]
        ];
        // let inverse_matrix_multiplication: [[u8; 4]; 4] = [
        //     [0x0E, 0x0B, 0x0D, 0x09],
        //     [0x09, 0x0E, 0x0B, 0x0D],
        //     [0x0D, 0x09, 0x0E, 0x0B],
        //     [0x0B, 0x0D, 0x09, 0x0E]
        // ];
        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    let gf = gf258(matrix_multiplication[x][z] as u16,matrix[z][y] as u16);
                    let res = result[x][y]^gf;
                    if debugging==true{
                        println!("________________________________________________________________________");
                        println!("matrix[{x}][{z}]\t\t\t:{m:08b} {m:x}",m=matrix[x][z]);
                        println!("matrix_multiplication[{z}][{y}]\t:{m:08b} {m:x}",m=matrix_multiplication[z][y]);
                        println!("gf258\t\t\t\t:{:08b}",gf);
                        println!("awal[{x}][{y}]\t\t\t:{:08b}",result[x][y]);
                        println!("xor\t\t\t\t:{:08b}",res);
                        
                    }
                    result[x][y] = res;
                    if debugging==true{
                        println!("stored[{x}][{y}]\t\t\t:{:08b}",result[x][y]);
                    }
                }
            }
        }
        result
    }
    // fungsi penghitungan irreducible polinomial
    fn gf258(x:u16,mut y:u16) -> u8 {      
        let p:u16 = 0b100011011;        
        let mut m = 0;         
        for _ in 0..8{
            m = m << 1;
            if (m & 0b100000000)!=0{
                m = m ^ p;
            }
                
            if (y & 0b010000000)!=0{
                m = m ^ x;
            }
            y = y << 1
        }
        m as u8
  
    }
    // fungsi enkripsi data matric 4x4 dengan rkey 4x4 balikin 4x4 yang sudah dienkripsi
    fn encryption(debugging: bool, mut matrix: [[u8; 4]; 4], rkeys: Vec<[[u8; 4]; 4]>) -> [[u8; 4]; 4] {
        // iterasi per rkey
        for (i,rkey) in rkeys.iter().enumerate() {
            // proses awal
            if i==0||i==1{
                // proses add round key
                matrix = add_round_key(matrix,*rkey);
            }
            // proses last round
            else if i==rkeys.len()-1 {
                // proses substitution box
                for x in 0..4 {
                    for y in 0..4 {
                        matrix[x][y] = substitution_box(debugging,matrix[x][y]);
                    }
                }
                // proses shift rows
                matrix = shift_rows(matrix);
                // proses mix column
                matrix = add_round_key(matrix,*rkey);
            // proses standar
            }else{
                // proses substitution box
                for x in 0..4 {
                    for y in 0..4 {
                        matrix[x][y] = substitution_box(debugging,matrix[x][y]);
                    }
                }
                // proses shift rows
                matrix = shift_rows(matrix);
                // proses mix column
                matrix = mix_columns(debugging,matrix);
                // proses add round key
                matrix = add_round_key(matrix,*rkey);
            }
            if debugging == true {
                println!("round {i}:\n{matrix:#?}\n");
                println!("round key {i}:\n{rkey:#?}\n");
            }
        }
        matrix
    }

    // This is the main program that executes process
    // change value for debugging
    // this is default value
    let debugging = true;
    let not_with_value = true;

    // take input
    let bytes_array = convert_input_value_to_bytes(debugging,take_input(not_with_value));

    // split key with its plain text array
    let key = bytes_array.get(0).unwrap();
    let data_array = bytes_array.iter().skip(1).collect::<Vec<_>>();

    // create round key
    let rkeys = key_expansion(debugging,key.to_vec());

    //  encrpted data
    let mut encrypted_data_array = Vec::new();
    // iterate over each arg
    for (index,data) in data_array.iter().enumerate() {
        //arg will be split to an array of 4x4 matrix
        if debugging == true {
            println!("\ndata: {index:?}");
        }
        let matrix_blocking = split_byte_array_to_an_array_of_4x4_matrix(debugging,data.to_vec());
        let mut encrypted = Vec::new();
        for matrix in matrix_blocking {
            let encryption_result = encryption(debugging,matrix,rkeys.clone());
            encrypted.push(encryption_result);
        }
        encrypted_data_array.push(encrypted);
    }

    // print result dalam json
    let begin = r#"{"#;
    let end = r#"}"#;
    // let koma = r#","#;
    print!("{begin} \"cyphertext\" : ");
    // for (index,data) in encrypted_data_array.iter().enumerate() {
    //     if index==encrypted_data_array.len()-1{
    //         print!("{data:?}");
    //     }
    //     else{
    //         print!("{data:?},");
    //     }
    // }
    print!("{encrypted_data_array:?}",);
    print!("{end}");
}