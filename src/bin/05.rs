fn main() {
    // bool = true false
    // == != > < >= <= 
    let x=32;
    let y =42;
    
    print!("{}",x ==y);
    print!("{}",x !=y);
    print!("{}",x < y);
    print!("{}",x > y);
    print!("{}",x >=y);
    print!("{}",x <=y); 

    print!("{}","abc" < "abcd"); //true 
    // print!("{}",3 < 3.14); // ERROR 3 = i && 3.14 = float

        let mut z =10;
        z += 2;
        z -= 4;
        z *= 4;
        z /= 4;

        print!("Le resultat z = {}", z);
        
}