/// Function for covert hours in minutes and seconds 
/// 
/// Insert a int value

pub fn convert(hour: i32){
    println!("Insert a hours for convert to seconds");

    let hour_to_sec: i32 = hour * 60 * 60 ;
    let hour_to_min: i32 = hour * 60;

    println!("Hours: {} minutes:{} seconds: {}", hour ,hour_to_min, hour_to_sec);

}