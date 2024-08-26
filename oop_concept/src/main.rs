fn main() {
    //if mut keyword is ahead of variable name, then the variable value can be change.
    let x = 5;
    println!("X is {x}"); 

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


}
