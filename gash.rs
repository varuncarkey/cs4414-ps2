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
use std::io::{Open, Read, Write, ReadWrite, Append, Truncate};
use std::io::signal::{Listener, Interrupt};
use std::io::pipe::PipeStream;
use std::str;


struct Shell {
    cmd_prompt: ~str,
    cwd: Path,
}




impl Shell {

    fn new(prompt_str: &str) -> Shell {
        Shell {
            cmd_prompt: prompt_str.to_owned(),
            cwd : os::getcwd(),
        }
    }
    
    fn run(&mut self) {
        let mut stdin = BufferedReader::new(stdin());
        spawn(proc(){
                    let mut listener = Listener::new();
                    listener.register(Interrupt);
                    loop{
                        match listener.port.recv() {
                            Interrupt => println!("Got Interrupt'ed"),
                            _ => (),
                        }
                    }
                }
            );
        let mut history : ~[~str] = ~[];
        if history.len()==0
        {
            let path=Path::new("history.txt");
            let mut file=match File::open_mode(&path,Append,ReadWrite)
            {
                Some(s) => s,
                None => File::create(&path).unwrap()

            };
            let total=file.read_to_str();
            //println!("{}",total);
            for temp in total.split('\n')
            {
                history.push(temp.to_str().to_owned());
            }
            
        }
        
        //print!("{}/", os::getcwd().display());
        
        loop {


            print!("{} : ", self.cwd.display());
            print(self.cmd_prompt);
            io::stdio::flush();
            
            let line = stdin.read_line().unwrap();
            let cmd_line = line.trim().to_owned();
            let program = cmd_line.splitn(' ', 1).nth(0).expect("no program");
            /*if line==~""
            {

            }
            else
            {*/
                history.push(line);
            //}
            
            
            
            
            //print!("{:s}",line);
        //if !done    
            match program {
            
                ""      =>  { continue; }
                "exit"  =>  {   
                                let path=Path::new("history.txt");
                                let mut file=match File::open_mode(&path,Truncate,ReadWrite)
                                {
                                    Some(s) => s,
                                    None => fail!("whoops! I'm sure this raised, anyways..")

                                };
                                for i in range(0,history.len())
                                {
                                    file.write_line(history[i]);
                                }
                                unsafe{ std::libc::funcs::c95::stdlib::exit(0);} 
                            }
                "cd"    =>  {

                            if cmd_line.splitn(' ',1).len() > 1 {
                                let rest: ~str = cmd_line.splitn(' ',1).nth(1).expect("").to_owned();
                                match rest
                                {
                                    _ => 
                                    {
                                            let path = Path::new(rest);
                                            if std::os::change_dir(&path)
                                            {
                                                self.cwd = os::getcwd();
                                            }
                                            else
                                            {
                                                println("Directory Doesnt exist");
                                            }
                                        
                                    }
                                    
                                }
                                
                            } else {
                            println("No directory was specified");
                            }
                          
                            //print!("rest: {:s}",rest);
                            //return;
                        }
                "history" =>{
                                for i in range(0, history.len()) { 
                                    print!("{} ",(i+1));
                                    println!("{:s}", history[i]);
                                }   
                            }
                
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

        if self.cmd_exists(program) {
           let mut process_done=false;
           for i in range(0,argv.len())
            {
                if argv[i]==~"<"||argv[i]==~">" || argv[i]==~"|"
                {
                
                process_done = true;
                    if argv[i]==~">"
                    {
                        let mut argv2=argv.clone().to_owned();
                        argv2.remove(0);
                        argv2.remove(0);
                        let f = match native::io::file::open(&argv[i+1].to_c_str(),
                                         Open, Write) {
                                                        Ok(f)  => f,
                                                        Err(e) => fail!("{}",e.to_str())
                                                    };
                        let fd = f.fd();
                        let mut options =run::ProcessOptions::new();
                        options.out_fd=Some(fd);
                        options.in_fd=Some(0);
                        options.err_fd=Some(2);
                        run::Process::new(program.to_owned(), argv.to_owned(),options);
                        process_done=true;

                    }
                    else if argv[i]==~"<"
                    {
                        let mut argv2= argv.clone().to_owned();
                        
                        argv2.remove(0);
                        argv2.remove(0);
                        
                        let filename=self.cwd.as_str().unwrap().to_owned()+"/"+argv[i+1];
                        println!("FILENAME: {}",filename);
                        let f = match native::io::file::open(&filename.to_c_str(),
                                         Open, Read) {
                                                        Ok(f)  => f,
                                                        Err(e) => fail!("{}",e.to_str())
                                                    };

                        let fd=f.fd();

                        let mut options =run::ProcessOptions::new();
                        options.in_fd =Some(fd);
                        options.out_fd=Some(1);
                        options.err_fd=Some(2);
                        
                        run::Process::new(program.to_owned(), argv2.to_owned(),options);
                        process_done=true;
                        
                    }

                    
                    
                    
                   
                }
                
            }
            
            if process_done{

                self.multiCulturalloop(Some(program.to_owned()), None, argv.to_owned());
            }

            else if !process_done
                {run::process_status(program, argv);}

        } else {
            println!("{:s}: command not found", program);
        }
    }
    
    fn multiCulturalloop(&mut self, prgm: Option<~str>, output: Option<~str>, mut argv: ~[~str]){
        match prgm{
            Some(prgm) => {
            let mut vec: ~[~str]=~[];
            for i in range(0, argv.len()){
                        
                if argv[i]==~"|"
                    {
                        let mut roundtwo=false;
                        match output.clone()
                        {
                            Some(out) => {roundtwo=true;}
                            None => {roundtwo=false;}
                        }
                    if !roundtwo{
                     let mut f;
                       f= match native::io::file::open(&".outfile.md".to_c_str(),
                                             Open,Write) {
                                                            Ok(f)  => f,
                                                            Err(e) => fail!("{}",e.to_str())
                                                        };
                        let fd=f.fd();
                        let mut options =run::ProcessOptions::new();
                            options.in_fd =Some(0);
                            options.out_fd=Some(fd);
                            options.err_fd=Some(2);
                        
                        let mut newProcess= run::Process::new(prgm.to_owned(), vec,options).unwrap();
                        let outputNew = newProcess.finish_with_output().output;
                        
                        let mut newVec: ~[~str] = ~[]; 
                        let prg=argv[i+1].clone();
                        for j in range(i+2, argv.len()){
                            newVec.push(argv[j].clone());
                        }
                        return self.multiCulturalloop(Some(prg), Some(~""), newVec);

                      }
                      else
                      {
                        
                        let f= match native::io::file::open(&".outfile.md".to_c_str(),
                                             Open,Read) {
                                                            Ok(f)  => f,
                                                            Err(e) => fail!("{}",e.to_str())
                                                        };
                        let fd1=f.fd();
                        
                        let r= match native::io::file::open(&".outfile2.md".to_c_str(),
                                             Open,Write) {
                                                            Ok(f)  => f,
                                                            Err(e) => fail!("{}",e.to_str())
                                                        };
                        let fd2=r.fd();
                        
                        let mut options =run::ProcessOptions::new();
                        options.in_fd =Some(fd1);
                        options.out_fd=Some(fd2);
                        options.err_fd=Some(2);
                        
                        let mut oldProcess= run::Process::new(prgm.to_owned(),vec,options).unwrap();
                        let outputNew = oldProcess.finish_with_output().output;
                        
                        run::process_status("rm", [~".outfile.md"]);
                        let path=Path::new(".outfile.md");
                        let path2=Path::new(".outfile2.md");
                        let mut file=File::open_mode(&path2, Open, Read);
                        let mut d=File::open_mode(&path, Open, Write);
                        d.write(file.read_to_end());
                        run::process_status("rm", [~".outfile2.md"]);
                        //let mut d = File::create(&Path::new(".outfile.md"));
                        let mut newVec: ~[~str] = ~[]; 
                        let prg=argv[i+1].clone();
                        for j in range(i+2, argv.len()){
                            newVec.push(argv[j].clone());
                        }
                        return self.multiCulturalloop(Some(prg), Some(~""), newVec);
                        
                      
                      }

                    } else{
                    
                        vec.push(argv[i].clone());
                        
                    }
            }
            let filename=".outfile.md";
                let f = match native::io::file::open(&filename.to_c_str(),
                                         Open, Read) {
                                                        Ok(f)  => f,
                                                        Err(e) => fail!("{}",e.to_str())
                                                    };

                let fd=f.fd();
                let mut options =run::ProcessOptions::new();
                            options.in_fd =Some(fd);
                            options.out_fd=Some(1);
                            options.err_fd=Some(2);
                let mut newProcess = run::Process::new(prgm.to_owned(), argv,options).unwrap();
                
                let outputNew=(newProcess.finish_with_output().output);//std::str::from_utf8(newProcess.output().read_to_end()).to_owned();
                run::process_status("rm", [~".outfile.md"]);
                        
                return;
            
            }
            None => {
            
               /* let mut vec: ~[~str]=~[];
                let prg= argv.remove(0);
                for k in range(0, argv.len()){
          
                    match argv[k]{
                    
                        ~"|" =>{
                            let filename=".outfile.md";
                            let file2=".outfile2.md";
                            let f = match native::io::file::open(&filename.to_c_str(),
                                             Open, Read) {
                                                            Ok(f)  => f,
                                                            Err(e) => fail!("{}",e.to_str())
                                                        };
                            let fd=f.fd();
                        
                            let r = match native::io::file::open(&file2.to_c_str(),
                                             Open, Write) {
                                                            Ok(f)  => f,
                                                            Err(e) => fail!("{}",e.to_str())
                                                        };
    
                            let fd1=r.fd();
                            let mut options =run::ProcessOptions::new();
                                options.in_fd =Some(fd);
                                options.out_fd=Some(fd1);
                                options.err_fd=Some(2);
                            let mut newProcess = run::Process::new(prg.to_owned(), vec,options).unwrap();
                            let outputNew=newProcess.finish_with_output().output;
                            //run::process_status("rm", [~".outfile.md"]);
                            //let mut d = File::create(&Path::new(".outfile.md"));
                            //d.write(outputNew);
                            println!("OUT PUT: {}", outputNew.to_str());
                            
                            let mut newVec: ~[~str] = ~[];
                            for j in range(k+1, argv.len()){
                                newVec.push(argv[j].clone());
                            }
                            return self.multiCulturalloop(None, Some(outputNew), newVec);
                           }
                          _ => {
                            vec.push(argv[k].clone());
                            }
                    }
                }*/
               /* let filename=".outfile.md";
                let f = match native::io::file::open(&filename.to_c_str(),
                                         Open, Read) {
                                                        Ok(f)  => f,
                                                        Err(e) => fail!("{}",e.to_str())
                                                    };

                let fd=f.fd();
                let mut options =run::ProcessOptions::new();
                            options.in_fd =Some(fd);
                            options.out_fd=Some(1);
                            options.err_fd=Some(2);
                let mut newProcess = run::Process::new(prg.to_owned(), vec,options).unwrap();
                
                let outputNew=(newProcess.finish_with_output().output);//std::str::from_utf8(newProcess.output().read_to_end()).to_owned();
                return;*/
            
            
            }
            
        }

    }
    
    
    fn cmd_exists(&mut self, cmd_path: &str) -> bool {
        let ret = run::process_output("which", [cmd_path.to_owned()]);
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
