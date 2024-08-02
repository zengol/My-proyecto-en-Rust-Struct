//Definición de estructura Aca deben ir todos los cam
struct Usuario{

    id: i64,
    nombre: String,
    localidad: String,
}

fn main() {
    let us1= Usuario{
        id:1,
        nombre:String::from("Fernando"),
        localidad: String::from("Jujuy"),
    };

    let us2= Usuario{
        id:2,
        nombre:String::from("Nombre"),
        localidad: String::from("Salta"),
    };

    println!("Id 1: {}", us1.id);
    println!("Nombre 1: {}", us1.nombre);
    println!("Id 2: {}", us2.id);
    println!("Nombre 2 : {}", us2.nombre);

    let us3= Usuario{
        id: 43,
        nombre: String::from("rocha"),
        localidad: String::from("jujuy"),
    };
    let us4= Usuario{
        id: 54,
        nombre: String::from("marco"),
        localidad: String::from("Salta"),
    };
    println!("Id 3: {}", us3.id);
    println!("Nombre: {}",us3.nombre);
    println!("Localidad: {}", us3.localidad);
    println!("Id 4: {}",us4.id);
    println!("Nombre: {}",us4.nombre);
    println!("Localidad {}", us4.localidad);

}