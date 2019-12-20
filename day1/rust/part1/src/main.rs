use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// A module named `my_mod`
mod my_module {
    use math::round;

    // Items in modules default to private visibility.
    pub fn get_fuel_for_mass(mass: u32) -> u32 
    {
        let divided= round::floor((mass/3) as f64,0) as u32;
        return divided - 2;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_fuel_for_mass_12() {
            assert_eq!(2, get_fuel_for_mass(12));
        }

        #[test]
        fn test_get_fuel_for_mass_14() {
            assert_eq!(2, get_fuel_for_mass(14));
        }

        #[test]
        fn test_get_fuel_for_mass_1969() {
            assert_eq!(654, get_fuel_for_mass(1969));
        }

        #[test]
        fn test_get_fuel_for_mass_100756() {
            assert_eq!(33583, get_fuel_for_mass(100756));
        }
        
    }
}



fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_fuel = 0;

    for line in reader.lines() {
        let mass = line?.parse::<u32>().unwrap();
        total_fuel += my_module::get_fuel_for_mass(mass);
        println!("adding module mass {}", mass);
    }

    println!("Final result is : {}", total_fuel);
    Ok(())
}