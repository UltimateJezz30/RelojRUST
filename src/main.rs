use std::fs::File;
use std::io::{self, BufReader, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Tiempo {
    min: i32,
    seg: i32,
    ms: i32,
}

fn main() -> io::Result<()> {
    let mut recmundial = Tiempo { min: 0, seg: 0, ms: 0 };

    let archivo = File::open("clasificacion.txt")
        .expect("Error de Apertura de Archivo");

    let mut buf_reader = BufReader::new(archivo);
    let mut contenido = String::new();
    buf_reader.read_to_string(&mut contenido)?;

    let mut vectcont: Vec<&str> = Vec::new();

    println!("{}", contenido);

    for i in contenido.split(", ") {
        if i.trim() != "" {
            vectcont.push(i.trim_end());
            println!(" {}", i.trim_end());
        }
    }

    println!("Total de elementos: {}", vectcont.len());

    if vectcont.len() >= 3 {
        recmundial.min = i32::from_str(vectcont[0]).unwrap_or(0);
        recmundial.seg = i32::from_str(vectcont[1]).unwrap_or(0);
        recmundial.ms = i32::from_str(vectcont[2]).unwrap_or(0);
    }

    println!("{}:{}:{}", recmundial.min, recmundial.seg, recmundial.ms);

    for i in &vectcont {
        print!(" {} ", i);

        if i.trim_end().len() <= 3 {
            if !i.is_empty() {
                recmundial.min = i32::from_str(&i.trim_end()).unwrap_or(0);
                print!("{}", recmundial.min);
                println!(" PASE ");
            }
        }
    }

    Ok(())
}
