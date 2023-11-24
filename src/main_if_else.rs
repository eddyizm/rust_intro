#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32), // todo!("Move `mileage: u32` field into `age` field - a tuple with two fields: an `Age` enum, u32");
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}


// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the Age ("New" or "Used") and mileage
// Return a tuple with the arrow `->` syntax
fn car_quality (miles: u32) -> (Age, u32) {
    //todo!("Add conditional expression: If car has accumulated miles, return tuple for Used car with current mileage");
    let quality: (Age, u32);
    if miles > 0 {
        quality = (Age::Used, miles)
    } else {
        quality = (Age::New, miles)
    }
    //todo!("Return tuple for New car with zero miles");
    return quality
}


// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(
    color: String,
    transmission: Transmission,
    convertible: bool,
    miles: u32,
) -> Car {
    // Show details about car order
    // - Check if order is for Used or New car, then check the roof type 
    // - Print details for New or Used car based on roof type
    
    //todo!("Add conditional expression: If car is Used age, then check roof type");
    let _age = car_quality(miles);

    if _age.1 == 0 {
        println!("Build a new car: {:?}, {}, Hard top, {} miles\n", transmission, color, miles); 
    } else if _age.0 == Age::Used {
        // todo!("Add conditional expression: If roof is a hard top, print details");
        println!("Prepare a used car: {:?}, {}, Hard top, {} miles\n", transmission, color, miles); 
    }
        
    // Use the values of the input arguments
    // All new cars always have zero mileage
    Car {
        color: color,
        motor: transmission,
        roof: convertible,
        age: car_quality(miles),
    }
}

fn main() {
    
    // Create car color array
    //let colors = ["Blue", "Green", "Red", "Silver"]; //todo!("Set the enum values: 0 = Blue, 1 = Green, 2 = Red, 3 = Silver");

    // Declare the car type and initial values
    //let mut car: Car = Car { color: String::from(colors[0]), motor: Transmission::Automatic, roof: true, age: (Age::Used, 700) }; // todo!("Create `car` as a `Car` struct");     
    //let mut engine: Transmission = Transmission::Manual; // todo!("Declare `engine` as a `Transmission` enum, initialize to `Manual`");

    // Order 3 cars, one car for each type of transmission

    // // Car order #1: New, Manual, Hard top
    // car = car_factory(String::from(colors[0]), engine, true, 0);
    // println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // // Car order #2: Used, Semi-automatic, Convertible
    // engine = Transmission::SemiAuto;
    // car = car_factory(String::from(colors[1]), engine, false, 100);
    // println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // // Car order #3: Used, Automatic, Hard top
    // engine = Transmission::Automatic;
    // car = car_factory(String::from(colors[2]), engine, true, 200);
    // println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
    
    /* next exercise */
    // Car order #1: New, Manual, Hard top
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);

    // Car order #2: Used, Semi-automatic, Convertible
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    // Car order #3: Used, Automatic, Hard top
    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}
