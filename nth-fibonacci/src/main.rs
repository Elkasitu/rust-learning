use std::io;

fn fibo(n: u32) -> u32
{
    if n == 0
    {
        return 0;
    }
    if n == 1
    {
        return 1;
    }

    let f1 = fibo(n - 2);
    let f2 = fibo(n - 1);
    return f1 + f2;
}

fn ask() -> u32
{
    loop
    {
        let mut n = String::new();
        println!("Which fibonacci number do you want? ");
        io::stdin().read_line(&mut n)
            .expect("Failed to retrieve input");

        let n: u32 = match n.trim().parse()
        {
            Ok(num) => num,
            Err(_)  => continue,
        };

        return n;
    }
}

fn main()
{
    let n = ask();
    let res = fibo(n);
    println!("The fibonacci number {} is {}", n, res);
}
