//use structopt::StructOpt;

//#[derive(StructOpt)]
//struct Cli {
    //pattern: String,
    //#[structopt(parse(from_os_str))]
    //path: std::path::PathBuf,
//}

//fn main() {
    //let args = Cli::from_args();
    //println!("{}, {:?}", args.pattern, args.path);

    //let content = std::fs::read_to_string(&args.path)
        //.expect("nao achei o file");

    //for line in content.lines() {
        //if line.contains(&args.pattern) {
            //println!("{}", line);
        //}
    //}

    //show_my_file();
//}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    //sem shortcut
    //let result = std::fs::read_to_string("teste.txt"); 
    //let content = match result {
        //Ok(content) => { content },
        //Err(error) => { return Err(error.into()); }
    //};

    let content = std::fs::read_to_string("teste.txt")?;
    println!("Ó o que tinha la: {}", content);
    Ok(())
}
