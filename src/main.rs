pub mod save;
pub mod mode;
pub mod config;

use std::{io::Error};
use config::Config;
use mode::Mode;
use rand::seq::SliceRandom;
use save::{FnPtr, Save, Saver, SaveParams, ISave};
use indoc::indoc;

fn main() {
    let config = 
        match Config::<String>::new(std::env::args().skip(1)) {
            Ok(x) => x,
            Err(e) => {
                let names_vec = Mode::<String>::names();

                    let mut names = vec![];
                    for name in names_vec {
                        names.push(format!("'{}'", name))
                    };

                    println!(indoc! {
                        "{} \n
                        Please choose a mode to run.\n
                        Acceptable Modes are are:
                        {:}

                        Eg: Run using `demorust2 <Mode> <Value>`\n
                        A string value may also be provided
                        Eg: 'demorust2 Saver Hello'"
                    },
                    e,
                    names.join(", ")); 

                    return
            },
    };
        
    match config.mode.match_mode() {
        Some(x) => x(config.value),
        None => ()
    };
}

fn run_save_direct(input: Option<String>) {
    let c = |x: &str| -> Result<(), Error>
    {  
        println!("Hello, world {}!", x);
        Ok(())
    };

    let opts_f: Vec<fn(x: &str) -> Result<(), Error>> = vec![test, test2, c];

    let mut opts_ptr:Vec<FnPtr<&str>>  = vec![
    str_func1,
    |x: &str| -> String
    {
        println!("Hello {}", &x);
        x.to_string()
    }];
    opts_ptr.push(str_func2);
    

    let f = opts_f.choose(&mut rand::thread_rng()).unwrap();
    let ptr = opts_ptr.choose(&mut rand::thread_rng()).unwrap();

    let t = Save::new(
        f,
        *ptr
    );

    let input_val = input.unwrap_or("Default".to_string());
    let r = &input_val;

    let val = format!("{}{}{}", input_val, " ", "f");

    t.execute_func(&val).unwrap();
    t.execute_t(format!("{}{}{}", r, " ", "t").as_str()).unwrap();
}

fn run_save_with_saver(input: Option<String>) {
    let saver = Saver::new(Save::new(
        |x| 
        {
            match x.is_cool {
                true => println!("You are cool!😎 You game me an input!"),
                false => println!("Not cool. You did not give me an input.😢")
            };
            Ok(())
        },
        |x| x.is_cool.to_string()
    ));

    saver.save(SaveParams
        {
            is_cool: match input { Some(_x) => true, None => false} 
        })
        .unwrap();
}

// Functions that may be passed in
fn str_func2(x: &str) -> String {
    println!("1,2,3... {}", x);
    x.to_string()
}

fn str_func1(x: &str) -> String {
    println!("Testing... {}", x);
    x.to_string()
}

fn test(x: &str) -> Result<(), Error> {
    println!("Testing... {}", x);
    Ok(())
}

fn test2(x: &str) -> Result<(), Error> {
    println!("1,2,3... {}", x);
    Ok(())
}