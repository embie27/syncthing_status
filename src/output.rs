static CONNECTION_ERROR: &str = "👎";

use crate::api_handler::*;

pub fn format_output(
        folders: &Result<Vec<Folder>, Box<dyn std::error::Error>>,
        name: &String,
        long_name: &String,
        is_last: bool,
    ) -> (String, String) {

    // let mut status = String::from("Ok");
    let mut status = String::from("👍");
    let mut file_string: String;

    let ln = format!("{} ", &long_name);
    file_string = format!("-------------------- {:-<25}\n", ln);

    match folders {
        Ok(folders) => {
            for f in folders.iter() {
                file_string = format!(
                    "{}{:>13}: {:<10} | Errors: {} \n",
                    file_string, f.label, f.state, f.errors
                );

                if &f.state != "idle" && &f.state != "" {
                    // emtpy string is when folder is paused
                    status = f.state.clone() // only keep last not-idle status
                }
            }
        }
        Err(_) => {
            file_string += &CONNECTION_ERROR.to_string();
            status = CONNECTION_ERROR.to_string();
        }
    }

    if is_last {
        (file_string, format!("{}: {}", name, status))
    } else {
        (file_string + "\n", format!("{}: {} - ", name, status))
    }
}
