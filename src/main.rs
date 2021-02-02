#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate migration_csv_to_postgresql;
extern crate postgres;
use chrono::offset::{TimeZone, Utc};
use postgres::{Connection, TlsMode};
use std::fs::File;
use std::io::{BufRead, BufReader};

use migration_csv_to_postgresql::is_dot;
use migration_csv_to_postgresql::is_lower;
use migration_csv_to_postgresql::is_middle_dash;
use migration_csv_to_postgresql::is_number;
use migration_csv_to_postgresql::is_two_words;
use migration_csv_to_postgresql::is_underscore;
use migration_csv_to_postgresql::is_upper;
use migration_csv_to_postgresql::is_upper_enie;

#[get("/")]
fn index() -> String {
    String::from("HELLO WORLD")
}

// struct User<'a> {
//     username: &'a str,
//     password: &'a str,
// }
// #[post("/", data = "<var>")]
// fn hello(var: User) -> String {
//     format!("{} {}", var.username, var.password)
// }

struct Persona {
    id: i32,
    //
    nombre: String,
    //
    identificacion: String,
    tipo_identificacion: String,
    //
    genero: String,
    estado_civil: String,
    fecha_nacimiento: String,
    //
    telefono: String,
    tipo_telefono: String,
    //
    direccion: String,
    email: String,
    observacion: Option<String>,
    validado: bool,
}

fn main() {
    // rocket::ignite().mount("/", routes![index, hello]).launch();
    create_table();

    // READ THE FILE
    let filename = "registros.csv";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.
        println!("{}. {}", index + 1, line);
        let line_split: Vec<&str> = line.split(';').collect();
        // println!("{:?}", line_split);
        let identificacion = line_split[0];
        let nombre = line_split[1];
        let genero = line_split[2];
        let estado_civil = line_split[3];
        let fecha_nacimiento = line_split[4];
        let telefono = line_split[5];
        let direccion = line_split[6];
        let email = line_split[7];

        let mut observacion = String::new();
        let mut validado = true;
        let mut tipo_identificacion = String::new();
        let mut tipo_telefono = String::new();
        /* IDENTIFICACION */
        match identificacion.len() {
            10 => {
                if check_cedula(&identificacion) {
                    tipo_identificacion.push_str("CEDULA");
                } else {
                    observacion.push_str("Cedula, no cumple con reglas, ");
                    validado = false;
                }
            }
            len => {
                let result = check_passport(&identificacion, len);
                if result == "true" {
                    tipo_identificacion.push_str("PASAPORTE");
                } else if result == "false" {
                    observacion.push_str("Pasaporte, no cumple con reglas, ");
                    validado = false;
                } else {
                    tipo_identificacion.push_str("CEDULA");
                    observacion.push_str(&result);
                    validado = false;
                }
            }
        }

        /* NOMBRE */
        let nombre = nombre.to_uppercase();
        let chars: Vec<char> = nombre.chars().collect();
        let mut name = String::new();
        for elem in chars {
            if !elem.is_ascii() {
                let elem_string = &elem.to_string();
                let ascii_word = elem_string.as_bytes();
                if ascii_word.contains(&195) {
                    if ascii_word.contains(&129) {
                        name.push('A');
                    } else if ascii_word.contains(&137) {
                        name.push('E');
                    } else if ascii_word.contains(&141) {
                        name.push('I');
                    } else if ascii_word.contains(&147) {
                        name.push('O');
                    } else if ascii_word.contains(&154) {
                        name.push('U');
                    } else {
                        name.push(elem);
                    }
                }
            } else {
                name.push(elem);
            }
        }
        let nombre = name;
        if is_two_words(&nombre) {
        } else {
            observacion.push_str("nombre invalido, ");
            validado = false;
        }
        /* GENERO */
        let genero = match genero {
            "M" => "M",
            "F" => "F",
            "NULL" => "NULL",
            _ => "NULL",
        };
        /* ESTADO CIVIL */
        const ESTADO_CIVIL: [&str; 6] = [
            "SOLTERO",
            "CASADO",
            "DIVORCIADO",
            "VIUDO",
            "EN UNION DE HECHO",
            "NULL",
            // "UNION LIBRE",
            // "SEPARADO",
        ];
        let estado_civil = estado_civil.to_uppercase();
        let mut result = String::new();
        for elem in ESTADO_CIVIL.iter() {
            if elem.to_string() == estado_civil {
                result.push_str(elem);
            }
        }
        if result.len() == 0 {
            result.push_str("NULL");
        }
        let estado_civil = result;
        /* FECHA DE NACIMIENTO */
        let date_split: Vec<&str> = fecha_nacimiento.split('-').collect();
        if date_split.len() == 3 {
            let year: i32 = date_split[0].parse().unwrap_or(0);
            let month: u32 = date_split[1].parse().unwrap_or(0);
            let day: u32 = date_split[2].parse().unwrap_or(0);
            let dt1 = Utc.ymd(year, month, day);
            let dt2 = Utc::now().date();
            let duration = dt2.signed_duration_since(dt1);
            let weeks = duration.num_weeks();
            let years = weeks / 52;
            if years >= 8 && years <= 95 {
            } else {
                observacion.push_str("no esta en el rango de edad, ")
            }
        }
        /* TELEFONO */
        let chars: Vec<char> = telefono.chars().collect();
        let mut is_valid = true;
        let mut tel = String::new();
        for elem in chars {
            if is_number(&elem.to_string()) {
            } else {
                is_valid = false;
            }
        }
        if is_valid {
            match telefono.len() {
                9 => {
                    let start_with = &telefono[..2];
                    let mut numeros_provincia: Vec<u32> = Vec::new();
                    for elem in 2..7 {
                        numeros_provincia.push(elem);
                    }
                    let start_with: u32 = start_with.parse().unwrap();
                    if numeros_provincia.contains(&start_with) {
                        tel.push_str("593");
                        tipo_telefono.push_str("CONVENCIONAL");
                    }
                    tel.push_str(telefono);
                }
                10 => {
                    let start_with = &telefono[..2];
                    if start_with == "09" {
                        tel.push_str("593");
                        tipo_telefono.push_str("CELULAR");
                    }
                    tel.push_str(telefono);
                }
                len => {
                    if len >= 6 {
                        tel.push_str(telefono);
                    }
                }
            }
        } else {
            observacion.push_str("telefono invalido, ");
            validado = false;
        }
        let telefono = tel;
        /* DIRECCION */
        if is_two_words(&direccion) {
        } else {
            observacion.push_str("direccion invalida, ");
        }
        /* EMAIL */
        let mut email = email;
        email = email.trim();
        let mut email = email.to_string();
        email.retain(|c| c != ' ');
        let mut email_split: Vec<&str> = email.split('@').collect();
        let mut email = String::new();
        if email_split.len() == 2 {
            if email_split[1].starts_with(".") {
                email_split[1] = &email_split[1][1..];
            }
            if email_split[0].ends_with(".") {
                email_split[0] = &email_split[0][..email_split[0].len() - 1];
            }
            let dominio_ext: Vec<&str> = email_split[1].split('.').collect();
            let dominio = dominio_ext[0];
            if dominio.len() >= 2 && dominio.len() <= 6 {
            } else {
                observacion.push_str("tamaño del dominio[email] es incorrecto, ");
                validado = false;
            }
            let chars: Vec<char> = email_split[0].chars().collect();
            let mut is_valid = true;
            for word in chars {
                let word_string = word.to_string();
                if is_number(&word_string)
                    || is_lower(&word_string)
                    || is_dot(&word_string)
                    || is_middle_dash(&word_string)
                    || is_underscore(&word_string)
                {
                } else {
                    is_valid = false;
                }
            }
            if is_valid {
                email.push_str(email_split[0]);
                email.push_str("@");
                email.push_str(email_split[1]);
            } else {
                for elem in email_split {
                    email.push_str(elem);
                }
                observacion.push_str("email invalido");
                validado = false;
            }
        }

        let data_to_save = Persona {
            id: 0,
            nombre: nombre.to_owned(),
            identificacion: identificacion.to_owned(),
            tipo_identificacion: tipo_identificacion.to_owned(),
            genero: genero.to_owned(),
            estado_civil: estado_civil.to_owned(),
            fecha_nacimiento: fecha_nacimiento.to_owned(),
            telefono: telefono.to_owned(),
            tipo_telefono: tipo_telefono.to_owned(),
            direccion: direccion.to_owned(),
            email: email.to_owned(),
            observacion: Some(observacion.to_owned()),
            validado: validado,
        };

        save_data(data_to_save);
    }
}

fn check_cedula(identificacion: &str) -> bool {
    const CONSUMIDOR_FINAL: &str = "9999999999";

    let mut numeros_valido_cedula: Vec<u32> = vec![30, 50, 80];
    for elem in 1..25 {
        numeros_valido_cedula.push(elem);
    }
    if is_number(&identificacion) {
        let ident_slice_two = &identificacion[..2];
        let ident_slice_two: u32 = ident_slice_two.parse().unwrap();
        if numeros_valido_cedula.contains(&ident_slice_two) || CONSUMIDOR_FINAL == identificacion {
            return true;
        }
    }
    false
}

fn check_passport(identificacion: &str, len: usize) -> String {
    if len >= 5 && len <= 20 {
        let chars: Vec<char> = identificacion.chars().collect();
        let mut is_valid = true;
        for word in chars {
            let word_string = word.to_string();
            if is_number(&word_string) || is_upper(&word_string) || is_upper_enie(&word_string) {
            } else {
                is_valid = false;
            }
        }
        if is_valid {
            if len >= 10 {
                let num_cedula = &identificacion[..10];
                if check_cedula(num_cedula) {
                    return "este número es una cedula y no un pasaporte. ".to_string();
                }
            }
            return "true".to_string();
        }
    }
    "false".to_string()
}

fn save_data(data_to_save: Persona) {
    let conn = Connection::connect(
        "postgresql://postgres:postgres@localhost:5432/migration",
        TlsMode::None,
    )
    .unwrap();
    conn.execute(
        "INSERT INTO persona (
            nombre,
            identificacion,
            tipo_identificacion,
            genero,
            estado_civil,
            fecha_nacimiento,
            telefono,
            tipo_telefono,
            direccion,
            email,
            observacion,
            validado
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        &[
            &data_to_save.nombre,
            &data_to_save.identificacion,
            &data_to_save.tipo_identificacion,
            &data_to_save.genero,
            &data_to_save.estado_civil,
            &data_to_save.fecha_nacimiento,
            &data_to_save.telefono,
            &data_to_save.tipo_telefono,
            &data_to_save.direccion,
            &data_to_save.email,
            &data_to_save.observacion,
            &data_to_save.validado,
        ],
    )
    .unwrap();
}

fn create_table() {
    let conn = Connection::connect(
        "postgresql://postgres:postgres@localhost:5432/migration",
        TlsMode::None,
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS persona (
            id                  SERIAL PRIMARY KEY,
            nombre              VARCHAR NOT NULL,
            email               VARCHAR NOT NULL,
            identificacion      VARCHAR,
            tipo_identificacion VARCHAR,
            genero              VARCHAR,
            estado_civil        VARCHAR,
            fecha_nacimiento    VARCHAR,
            telefono            VARCHAR NOT NULL,
            tipo_telefono       VARCHAR,
            direccion           VARCHAR,
            observacion         VARCHAR,
            validado            BOOLEAN 
        )",
        &[],
    )
    .unwrap();
}

// for row in &conn
//     .query("SELECT id, name, data FROM person", &[])
//     .unwrap()
// {
//     let person = Person {
//         id: row.get(0),
//         name: row.get(1),
//         data: row.get(2),
//     };
//     println!("Found person {}", person.name);
// }
