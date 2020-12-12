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