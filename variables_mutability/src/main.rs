fn main() {
    let mut a = 1; // como a una variable inmutable no se le puede agregar un valor posterior al declarado en LET
    // entonces hacemos uso de la palabra reservada MUT para decirle que puede mutar o cambiar su valor
    print!("the value of a is {}", a);
    a = 2;
    print!("the value of a is {}", a);
}
