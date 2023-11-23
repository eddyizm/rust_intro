#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    mileage: (Age, u32), // todo!("Move `mileage: u32` field into `age` field - a tuple with two fields: an `Age` enum, u32");
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
fn car_quality(miles: u32) -> (Age, u32) {
    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    let quality = (Age::New, miles);
    //= todo!("Set the `Age` value to \"New\", set the mileage using the `miles` input argument");

    // Return the completed tuple to the caller
    return quality;
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
    // Use the values of the input arguments
    // All new cars always have zero mileage
    Car {
        color: color,
        motor: transmission,
        roof: convertible,
        mileage: car_quality(miles),
    }
}

fn main() {
    // Create car color array
    let colors = ["Blue", "Green", "Red", "Silver"]; //todo!("Set the enum values: 0 = Blue, 1 = Green, 2 = Red, 3 = Silver");

    // Declare the car type and initial values
    let mut car: Car = todo!("Create `car` as a `Car` struct");     
    let mut engine: Transmission = todo!("Declare `engine` as a `Transmission` enum, initialize to `Manual`");
}