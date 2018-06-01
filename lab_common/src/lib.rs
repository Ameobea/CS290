use std::fmt::Debug;
use std::io::stdin;
use std::str::FromStr;

pub fn debug<T: Debug>(x: T) -> String { format!("{:?}", x) }

pub fn get_user_input<T: FromStr>(prompt_opt: Option<&str>) -> T {
    let stdin_inst = stdin();
    if let Some(prompt) = prompt_opt {
        println!("{}", prompt);
    }

    let mut buf = String::new();
    stdin_inst.read_line(&mut buf).expect("Unable to read line from stdin!");
    match buf.parse() {
        Ok(parsed) => parsed,
        Err(_) => {
            println!("Error while parsing the supplied input; please try again.");
            get_user_input::<T>(prompt_opt)
        }
    }
}
