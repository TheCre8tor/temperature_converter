use std::io;
// use std::cmp;

fn main() {
    /* Algorithms -->
       1. Water boils at 100 degrees Celsius or 212 degrees Fahrenheit, or 373.15 Kelvins.
       2. Water freezes at 0 degrees Celsius or 32 Degrees Fahrenheit, or 273.15 Kelvins.
       3. 0 degrees Celsius is equal to 32 degrees Fahrenheit.
          -> The basic formula is (°C × 9/5) + 32 = °F.
       4. The basic formula for converting Fahrenheit into Celsius is (°F − 32) × 5/9 = °C.

     */

     /* Greet the user and ask them what to convert
        They can either choose F to C or C to F
      */

      println!("\n###---- 6350 Temperature Converter ----###\n");
      println!("Enter F to convert Fahrenheit to Celsius.");
      println!("Enter C to convert Celsius to Fahrenheit.\n");

      loop {
        let mut user_input = String::new();
        
        io::stdin().read_line(&mut user_input).expect("Can not read user input");
  
        let user_input = user_input.trim().to_lowercase();
  
        if user_input.len() == 1 && (user_input == "f" || user_input == "c") {
            if user_input == "f" {
                fahrenheit_to_celsius();
                break;
            } else {
                celsius_to_fahrenheit();
                break;
            }
        } else {
            println!("Converter only accept F or C:");
            continue;
        } 
      }
     
}

fn fahrenheit_to_celsius() {
    loop {
       let mut fahrenheit = String::new();

       println!("Enter Fahrenheit °F:");
       io::stdin().read_line(&mut fahrenheit).expect("Error has occured!");
    
       let response = match fahrenheit.trim().parse::<f32>() {
           Ok(value) => value,
           Err(_) => {
               println!("Only numbers are required!\n");
               continue;
           }
       };
    
       let celsius: f32 = (response - 32.0) * 5.0/9.0;

       println!("Fahrenheit to Celsius: {}°C\n", celsius);
       break;
    }
}


fn celsius_to_fahrenheit() {
    println!("Print Celsius");
}
