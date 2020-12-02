#[macro_export]
macro_rules! challenge {
    ($day_number:literal, $statement:stmt) => {
        println!("----------- Running day {} challenge -----------", $day_number);
        $statement
        println!("----------- Day {} challenge over --------------", $day_number);
    };
}