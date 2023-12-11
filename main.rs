use rand :: Rng; //rand = "0.8.4" (Cargo.toml)
use std ::io; //input
use std ::cmp::Ordering; //Compare

fn main()
{
    println!("Number Guessing Game");
    let private_number = rand::thread_rng().gen_range(1..6); // 1 to 5
    //println!("Private Number: {}",private_number); //int

    loop
    {
        //user input
        println!("Enter Your Guess In Range Of 1 To 5");
        let mut public_number = String::new(); 
        io::stdin().read_line(&mut public_number)
        //validation
            .expect("Invalid Guess");
        println!("Public Number: {}",public_number); //String

        //String to int
        let public_number:u32 = public_number.trim().parse()
            .expect("Enter Integer");
        //println!("{}",public_number+1); //for int otw error //for checking conversion

        //compare
        match public_number.cmp(&private_number)
        {
            Ordering::Less => println!("Less Than The Private Number"),
            Ordering::Greater => println!("Greater Than The Private Number"),
            Ordering::Equal => 
            {
                println!("Number Is Equal To Private Number: {}",private_number);
                println!("Winner");
                break;
            },
        }
    }
}