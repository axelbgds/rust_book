// Variable and Mutabilite

fn main() {
    // Not possible to change var without mut
    let mut foo:i32 = 10;
    println!("Result from foo is :{foo}");

    foo = 20;
    println!("Result from foo is :{foo}");

    // convention name const CAMEL_CASE + type i32 - i64
    const FOO_BAR: i32 = 5;
    println!("Result from foo is :{FOO_BAR}");
    
    // shadow win 
    let bar= 14;
    let bar = bar +1;
    // depend on your scop
    {
        // bar = 9
        let bar = bar -6;
        println!("Value bar int scope :{bar}");
    }
    // bar = 15
    println!("Value bar ext scope :{bar}");


}