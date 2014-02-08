//
// gash.rs
//
// Starting code for PS2
// Running on Rust 0.9
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu, David Evans
// Version 0.4
//

extern mod extra;
extern mod native;
use std::{io, run, os};
use std::io::File;
use std::io::buffered::BufferedReader;
use std::io::stdin;
use extra::getopts;
use std::io::Writer;
use std::io::{Open, Read, Write,};
use std::io::pipe::PipeStream;
struct Shell {
    cmd_prompt: ~str,
}

impl Shell {
    fn new(prompt_str: &str) -> Shell {
        Shell {
            cmd_prompt: prompt_str.to_owned(),
        }
    }
    
    fn run(&mut self) {
        let mut stdin = BufferedReader::new(stdin());
        
        loop {
            print(self.cmd_prompt);
            io::stdio::flush();
            
            let line = stdin.read_line().unwrap();
            let cmd_line = line.trim().to_owned();
            let program = cmd_line.splitn(' ', 1).nth(0).expect("no program");
            
            //print!("{:s}",rest);
            match program {
                ""      =>  { continue; }
                "exit"  =>  { return; }
                "cd"    =>  {
                                let rest: ~str = cmd_line.splitn(' ',1).nth(1).expect("").to_owned();
                                match rest
                                {
                                    _ => 
                                    { 
                                        let path = Path::new(rest);
                                        if std::os::change_dir(&path)
                                        {
                                        }
                                        else
                                        {
                                            println("Directory Doesnt exist");
                                        }
                                        
                                    }
                                    
                                }
                          
                            //print!("rest: {:s}",rest);
                            //return;
                        }
                "history" =>{return;}
                _       =>  { self.run_cmdline(cmd_line); }
            }
        }
    }
    
    fn run_cmdline(&mut self, cmd_line: &str) {
        let mut argv: ~[~str] =
            cmd_line.split(' ').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();
        
        if argv.len() > 0 {
            let program: ~str = argv.remove(0);
            
            if argv.len()>0 && argv[argv.len()-1]==~"&"
            {
                let program2: ~str = program.clone();
                let argv2: ~[~str] = argv.clone();
                spawn(proc() 
                {
                let mut self2= Shell::new("");                
                self2.run_cmd(program2, argv2);
                });
            }
            else {self.run_cmd(program, argv);}
        }
    }
    
    fn run_cmd(&mut self, program: &str, argv: &[~str]) {
        //println("CAME HERE");
       // let mut process:(std::option::Option<std::run::Process>)=None;
        if self.cmd_exists(program) {
           let mut process_done=false;
           for i in range(0,argv.len())
            {
                if argv[i]==~"<"||argv[i]==~">" || argv[i]==~"|"
                {
                    if argv[i]==~">"
                    {
                        let f = match native::io::file::open(&argv[i+1].to_c_str(),
                                         Open, Write) {
                                                        Ok(f)  => f,
                                                        Err(e) => fail!("{}",e.to_str())
                                                    };
                        let fd = f.fd();
                        let mut options =run::ProcessOptions::new();
                        options.out_fd=Some(fd);
                        let mut newprocess=run::Process::new(program.to_owned(), argv.to_owned(),options).unwrap();
                        let over=newprocess.finish();
                        if over.success()
                        {
                            process_done=true;
                        }
                        //f.close();

                    }
                    else if argv[i]==~"<"
                    {
                        println!("argsv {}",argv[i+1].clone());
                        let path =Path::new(argv[i+1].clone());
                        let mut file = File::open(&path);
                        //print!("GOT HERE DID FD WORK {}",f.eof().to_str());
                        let mut options =run::ProcessOptions::new();
                        //options.in_fd=Some(fd);
                        let mut newprocess=run::Process::new(program.to_owned(), argv.to_owned(),options).unwrap();
                        newprocess.input().write(file.read_to_end());
                        let over=newprocess.finish();
                        if over.success()
                        {
                            process_done=true;
                        }

                    }
                }
            }
            if !process_done
                {run::process_status(program, argv);}

        } else {
            println!("{:s}: command not found", program);
        }
    }
    
    fn cmd_exists(&mut self, cmd_path: &str) -> bool {
        let ret = run::process_output("which", [cmd_path.to_owned()]);
        println("HERE MAYBE");
        return ret.expect("exit code error.").status.success();
    }
}

fn get_cmdline_from_args() -> Option<~str> {
    /* Begin processing program arguments and initiate the parameters. */
    let args = os::args();
    let opts = ~[
        getopts::optopt("c")
    ];
    
    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_err_msg()) }
    };
    
    if matches.opt_present("c") {
        let cmd_str = match matches.opt_str("c") {
                                                Some(cmd_str) => {cmd_str.to_owned()}, 
                                                None => {~""}
                                              };
        return Some(cmd_str);
    } else {
        return None;
    }
}

fn main() {
    let opt_cmd_line = get_cmdline_from_args();
    
    match opt_cmd_line {
        Some(cmd_line) => Shell::new("").run_cmdline(cmd_line),
        None           => Shell::new("gash > ").run()
    }
}
