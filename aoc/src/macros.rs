#[macro_export]
macro_rules! challenge {
    ($day_number:literal, $part:literal, $statement:stmt) => {
        println!("----------- Running day {} part {} challenge -----------", $day_number, $part);
        $statement
        // println!("----------- Day {} challenge over ----------------------", $day_number);
    };
}

#[macro_export]
macro_rules! str_to_i32 {
    ($string:expr) => {
        $string.to_string().parse::<i32>().unwrap();
    };
}

#[macro_export]
macro_rules! read_input {
    ($input:expr, $file_path:literal) => {
        $input.unwrap_or_else(|| {
            read_to_string($file_path)
                .expect("Could not open input file. Does it exist?")
        });
    };
}