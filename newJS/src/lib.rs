use suborbital::runnable::*;

struct NewJS{}

impl Runnable for NewJS {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let in_string = String::from_utf8(input).unwrap();
    
        Ok(String::from(format!("hello {}", in_string)).as_bytes().to_vec())
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &NewJS = &NewJS{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
