fn main(){

    // agrupa diferentes tipos
    // valores seprados por comas entre los parentesis
    // destructuracion rompe una tupla en partes
    // acceso a los elementos usando el . + el indice
    let tupla: (i32, i32, char) = (1, 2, 'Z'); // diferentes tipos de datos

    let (x, y ,z) = tupla;
    let a = tupla.0; // acceder a elemento de la tupla

    println!("x = {}, y = {}, z = {} ", x, y, z);
    println!("A = {}", a);
}