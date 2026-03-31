fn main() {
    println!("Hello, world!");
    let n = 7;
    let result = fibonacci(n);
    println!("le numero {} de la suite de fibonnaci est {}", n, result);
}

fn fibonacci(indice: u32) -> u32
{
    if indice == 0 {
        return 0 as u32;
    }
    else if indice == 1 {
        return 1 as u32;
    }
    else{
        let mut old_fib_value: u32 = 0;
        let mut fib_value: u32 = 1;
        for i in 3..=indice {
            let temp = old_fib_value + fib_value;
            old_fib_value = fib_value;
            fib_value = temp;
            println!("à l'indice {} fib_value {} et old_fib_value {}", i, fib_value, old_fib_value);
        }
        return fib_value;
    }
}