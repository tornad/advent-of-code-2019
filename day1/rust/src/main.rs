use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// A module named `my_mod`
mod day1_part1 {
    use math::round;

    // Items in modules default to private visibility.
    pub fn get_fuel_for_mass(mass: i64) -> i64 
    {
        let divided= round::floor((mass/3) as f64,0) as i64;
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


// A module named `my_mod`
mod day1_part2 {
    use math::round;
    use std::cmp;

    // Items in modules default to private visibility.
    pub fn get_fuel_for_mass(mass: i64) -> i64 
    {
        //println!("Tring with mass {}",mass);
        let divided = round::floor((mass/3) as f64,0) as i64;
        let mut needed_fuel = cmp::max((divided - 2) as i64, 0 ) as i64;
        if needed_fuel > 0 {
            //println!("Calling recursive {}",needed_fuel);
            needed_fuel += get_fuel_for_mass(needed_fuel);
        }
        return needed_fuel;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_fuel_for_mass_14() {
            assert_eq!(2, get_fuel_for_mass(14));
        }

        #[test]
        fn test_get_fuel_for_mass_1969() {
            assert_eq!(966, get_fuel_for_mass(1969));
        }

        #[test]
        fn test_get_fuel_for_mass_100756() {
            assert_eq!(50346, get_fuel_for_mass(100756));
        }
        
    }
}

fn main() -> io::Result<()> {
    day1_part1();
 
    day1_part2();
 
    Ok(())
}

fn day1_part1() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_fuel = 0;

    for line in reader.lines() {
        let mass = line?.parse::<i64>().unwrap();
        total_fuel += day1_part1::get_fuel_for_mass(mass);
        //println!("adding module mass {}", mass);
    }
    println!("Final result for day1 part1 is : {}", total_fuel);

    Ok(())
}

fn day1_part2() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_fuel = 0;

    for line in reader.lines() {
        let mass = line?.parse::<i64>().unwrap();
        total_fuel += day1_part2::get_fuel_for_mass(mass);
        //println!("adding module mass {}", mass);
    }
    println!("Final result for day1 part1 is : {}", total_fuel);

    Ok(())
}
