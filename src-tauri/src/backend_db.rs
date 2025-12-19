use rusqlite::Connection;
use serde::Serialize;

use tauri::command;

#[derive(Debug, Serialize)]
pub struct Dispositivos {
    pub id: i32,
    pub nombre_dispositivo: String,
}

#[derive(Debug, Serialize)]
pub struct UsuarioApp {
    pub nombre_usuario: String,
}

#[command]
pub fn consultas_db() -> Result<Vec<Dispositivos>, String> {
    let conn = Connection::open("remit_data.db").map_err(|e| e.to_string())?;

    //NOTA!, en lugar de utiliar un simple "?", utilio un map_err(|e| e.to_string())?
    // para que el frontend pueda manejar el error como un string

    //recuperar la lista de dispositivos
    let mut stmt = conn
        .prepare("SELECT * FROM dispositivos")
        .map_err(|e| e.to_string())?;

    let iterador_dispositivos = stmt
        .query_map([], |row| {
            Ok(Dispositivos {
                id: row.get(0)?,
                nombre_dispositivo: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut dispositivos = Vec::new();
    for dispositivo in iterador_dispositivos {
        dispositivos.push(dispositivo.map_err(|e| e.to_string())?);
    }

    Ok(dispositivos)
}

#[command]
pub fn user_app() -> Result<String, String> {
    let conn = Connection::open("remit_data.db").map_err(|e| e.to_string())?;
    if let Err(e) = init_db() {
        eprintln!("Error al inicializar la base de datos: {:?}", e);
    }; //inicializar la base de datos, el _ es para ignorar el resultado

    //Obtener el nombre del usuario actual, de la APP

    let mut res = String::from("nada");

    //Saber si el usuario se ha establecido un nombre
    let existe_nombre: i8 = conn
        .query_row(
            "SELECT count(*) FROM usuario_app",
            [],
            |row| Ok(row.get(0)?),
        )
        .map_err(|e| e.to_string())?;

    //si el usuario no tiene nombre, valió queso, digo, mostramos el nombre del dispositivo
    if username_exists(&conn) {
        //obtener el nombre del dispositivo
        let nombre_dispositivo = match hostname::get() {
            Ok(host) => host,
            Err(e) => {
                eprintln!("Error al obtener el nombre del dispositivo: {:?}", e);
                return Err("Error al obtener el nombre del dispositivo".to_string());
            }
        };

        //convertir el nombre del dispositivo a string
        res = match nombre_dispositivo.into_string() {
            Ok(nombre) => String::from(nombre),
            Err(e) => {
                eprintln!(
                    "Error al convertir el nombre del dispositivo a string: {:?}",
                    e
                );
                String::from("Error")
            }
        };

    // si no, mostremos el nombre que registrado en la base de datos
    } else {
        let usuario_app = conn
            .query_row("SELECT nombre_usuario FROM usuario_app", [], |row| {
                Ok(UsuarioApp {
                    nombre_usuario: row.get(0)?,
                })
            })
            .map_err(|e| e.to_string())?;
        res = String::from(usuario_app.nombre_usuario);
    }
    Ok(res)
}

#[command]
pub fn change_username(new_name: String) -> Result<(), String> {
    let conn = Connection::open("remit_data.db").map_err(|e| e.to_string())?;

    println!("nombre entrante: {:?}", new_name);

    //si usuario_exists es true, significa que no existe el usuario y se debe crear
    if username_exists(&conn) {
        println!("nombre creado");
        let mut stmt = conn
            .prepare("INSERT INTO usuario_app (nombre_usuario) VALUES (?1)")
            .map_err(|e| e.to_string())?;

        stmt.execute([new_name])
            .map_err(|e| format!("RUST:: {}", e))?;
    }
    //si usuario_exists es false, significa que existe el usuario y se debe actualizar
    else {
        println!("nombre actualizado");
        let mut stmt = conn
            .prepare("UPDATE usuario_app SET nombre_usuario = ?1")
            .map_err(|e| e.to_string())?;

        stmt.execute([new_name])
            .map_err(|e| format!("RUST:: {}", e))?;
    }

    Ok(())
}

//----- FUNCIONES PRIVADAS -----//

//verificar si existe un nombre de usuario
fn username_exists(conn: &Connection) -> bool {
    let existe_nombre: Result<i8, rusqlite::Error> =
        conn.query_row("SELECT count(*) FROM usuario_app", [], |row| row.get(0));

    // el resultado 0 significa que no existe nombre de usuario
    match existe_nombre {
        Ok(count) => count == 0,
        Err(e) => {
            eprintln!("Error al obtener el nombre del usuario: {:?}", e);
            false
        }
    }
}

//inicializar la base de datos
//solo se ejecuta la primera vez que se ejecuta la app
fn init_db() -> Result<(), String> {
    let conn = Connection::open("remit_data.db").map_err(|e| e.to_string())?;

    //comprobar que no existen tablas
    let tablas_count: i8 = conn
        .query_row("SELECT count(*) from sqlite_master", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    if tablas_count == 0 {
        //crear la tabla de usuario_app
        conn.execute(
            "CREATE TABLE IF NOT EXISTS 'usuario_app'(
        id INTEGER PRIMARY KEY NOT NULL,
        nombre_usuario TEXT NOT NULL)
        ;",
            [],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}
