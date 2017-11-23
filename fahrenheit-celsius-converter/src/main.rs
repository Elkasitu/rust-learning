use std::io;

fn main()
{
    println!("Welcome to the F<->C converter!");

    let mut val = String::new();
    let mut uom = String::new();

    loop
    {
        println!("Please enter a value: ");

        io::stdin().read_line(&mut val)
            .expect("Failed to retrieve input");

        let val: f64 = match val.trim().parse()
        {
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("Please enter a unit of measure (c/f): ");

        io::stdin().read_line(&mut uom)
            .expect("Failed to retrieve input");

        let uom = uom.trim();

        if uom == "c"
        {
            let res = val * (9.0 / 5.0) + 32.0;
            println!("{}{} is equal to {}{}", val, "c", res, "f");
            break;
        }
        else if uom == "f"
        {
            let res = (val - 32.0) * (5.0 / 9.0);
            println!("{}{} is equal to {}{}", val, "f", res, "c");
            break;
        }
    }
}
