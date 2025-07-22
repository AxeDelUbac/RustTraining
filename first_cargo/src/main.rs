use std::iter::Zip;

fn main() {

    // ggg
    let mut foo = 42;
    println!("The value of foo is: {foo}");

    foo = 20;
    println!("The value of foo is: {foo}");

    const FOO_VALUE: i32 = 10;
    println!("The value of FOO_VALUE is: {FOO_VALUE}");

    let bar = 100;
    let bar = bar + 50;

    {
        let bar = bar * 2;
        println!("The value of bar in the inner scope is: {bar}");
    }

    println!("The value of bar is: {bar}");

    println!("{}, {}", "Hello", "World");
    let x = 10;
    let y = 5;
    println!("{}", x + y);
    println!("{}", x * y);
    println!("{}", x / y);
    println!("{}", x - y);

    let total = x + y;
    println!("The total is: {total}");

    let a: i16 = 5;
    let b: u8 = 255;

    let dec = 2.5;
    println!("{}", dec);

    let bool = true;
    println!("{}", bool);

    let ch = 'a';
    println!("{}", ch);

    let mut c= 5;
    c += 1;
    println!("{}", c);
    c *= 2;
    println!("{}", c);

    let d = -10;
    let condition = d > 5;
    if d > 0 
    {
        println!("d is positive");
    }
    else {
        println!("d is not positive");  
    }
    println!("la condition est {}", condition);

    let mut g = 10;
    while g < 20 {
        g += 1;
        if g % 3 != 0 
        {
            continue;
        }
        if g * g > 60 {
            break;
        }
        println!("g is now: {}", g);
    }

    for x in 1..11
    {
        println!("x is now: {}", x);
    }

    let x = ["apple", "banana", "cherry"];

    println!("{} {} {} {}", x[0], x[1], x[2], x.len());

    let mut y = vec!["hello", "world", "from", "vector"];
    println!("Length of y: {} !", y.len());

    let o = ["";0];
    let mut p = vec![3.14;5];

    println!("{:?} and {:?}", o, p);

    let q = {
        let s = 10;
        s + 1
    };
    println!("la valeur de q est {} !", q);

    let j = foo_fn(20);
    println!("la valeur de j est {} !", j);

    let tuple: (i32, f64, bool) = (5, 3.14, true);
    let (tx, ty, tz) = tuple;
    println!("Tuple values: {tx}, {}, {}", ty, tz);




    let first_dog = Dog{
        name: String::from("Patoux"),
        age: 5,
        gender: String::from("Male"),
        under_10: true
    };

    println!("{:#?}", first_dog);

    println!("The name of the dog is: {}, and his age is: {}", first_dog.name, first_dog.age);

    register_dog(String::from("Rex"), 3);

    let mut x = String::from("Hello");
    x.push_str(", World!");

    println!("{x}");
    let y = "hello";

    println!("{}", y);

}

fn foo_fn(x: i32)-> i32 {
    println!("la valeur de x est {} !", x);
    x + 1
}

fn register_dog(name: String, age: i32) -> Dog {
    Dog {
        name: name,
        age: age,
        gender: String::from("Unknown"),
        under_10: false
    }
}

    #[derive(Debug)]
    struct Dog {
        name: String,
        age: i32,
        gender: String,
        under_10: bool
    }