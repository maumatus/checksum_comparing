use std::fs::File;
use std::io::prelude::*;

// Partimos creando un checksum compare con "Strings" completos"
// Luego haremos refactor para que diga cual archivo no es igual
fn main() {
    let mut file_a = File::open("md5sums_1.txt")
    .expect("Archivo no encontrado");
    let mut file_b = File::open("md5sums_2.txt")
    .expect("Archivo no encontrado");
    
    let mut data_a = String::new();
    let mut data_b = String::new();
    
    file_a.read_to_string(&mut data_a)
    .expect("Error leyendo el archivo");
    
    file_b.read_to_string(&mut data_b)
    .expect("Error leyendo el archivo");
    
    if data_a == data_b {
        println!("Grupo de archivos coinciden");
    } else {
        println!("Grupo de archivos no coinciden");
    }
}
