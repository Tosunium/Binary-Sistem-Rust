fn main() {
    
    let tosun = "1100010 1110101 100000 1110100 1101111 1110011 1110101 1101110 100000 1101011 1101001 1101101 1100101 100000 1100111 1101111 1110011 1110101 1101110";


    let tosunium = oba(tosun);
    

    print!("{} ", tosunium );
}

fn oba(tosun: &str) -> String{

    tosun.split_whitespace()
        .map(|tosun| u8::from_str_radix(tosun, 2).unwrap() as char)  
        .collect()  }