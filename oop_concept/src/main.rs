fn main() {
    //if mut keyword is ahead of variable name, then the variable value can be change.
    let x = 5;
    println!("X is {x}");

    //prefix by i then its signed integer and if its u then its unsigned integer
    // signed integer can be +1 or -1

    // easy readability we can use _
    let s = 1_00_00_000;

    let bin_four = 0b100;

    println!("{}",bin_four);

    //Shadowing
    let x = 20;{
        let x = "14";
        println!("X inside scope is {x}");
    }
    println!("X is {x}");

    // cant use mut keyword with const variable.
    // always give type
    const PI:i32 = 4;
    println!("PI value is {PI}");

    let tup = (500,12,12.56,"ABC"); 
    let (a,b,c,d) = tup;
    println!("A : {a}");
    println!("1st value is {}",tup.1);

    let arr =  [1,2,3];


}
