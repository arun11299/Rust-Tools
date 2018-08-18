use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::env;
use std::thread;
use std::time::Duration;
use std::io::BufReader;

const CHARS: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 
'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

const LINES: u8 = 50;

fn get_arg(pos : usize, args: &mut Vec<String>) -> Result<u32, String>
{
    args[pos].parse::<u32>()
        .map_err(|err| err.to_string())
    /*
    args.nth(pos)
        .ok_or("Please give at least one argument".to_owned())
        .and_then(|arg| {
            arg.parse::<u32>()
               .map_err(|err| err.to_string())
        })
    */
}

fn write_worker(id : u8, wr_size : usize)
{
    let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("atomic-log-test")
                .unwrap();

    let ch = &CHARS[id as usize];

    let mut wr_str = std::iter::repeat(ch).take(wr_size).collect::<String>();
    wr_str.push('\n');

    for i in 0 .. LINES {
        std::thread::sleep(Duration::from_millis(10));
        file.write(wr_str.as_bytes());
    }
}

fn verify_file(wr_sz : usize) -> bool
{
    let mut file = OpenOptions::new()
                    .read(true)
                    .open("atomic-log-test")
                    .unwrap();
    
    for line in BufReader::new(file).lines() {
        match (line) {
            Ok(l) => {
                if l.len() != wr_sz {
                    println!("{} {}", l.len(), wr_sz);
                    return false;
                }
                let ch = l.chars().next().unwrap();
                let all_same = l.chars().all(|elem| elem == ch);
                if !all_same {
                    return false;
                }
            },
            Err(_) => {
                println!("Error readling line");
                return false;
            }
        }
    }
    
    return true;
}

fn main()
{
    let mut argv : Vec<_> = env::args().collect();
    let num_threads = get_arg(1, &mut argv)
                      .map(|n| {
                          if n > 26 { 26 } else { n }
                      })
                      .map_err(|err| 0);
    let write_sz_res = get_arg(2, &mut argv)
                   .map(|n| {
                       n
                   })
                   .map_err(|err| println!("{}", err));
    
    let nthread = match num_threads {
        Ok(val) => val,
        Err(_)  => std::process::exit(-1),
    };
    let write_sz = match write_sz_res {
        Ok(val) => val,
        Err(_) => std::process::exit(-1),
    };
                    
    println!("Running with threads {}", nthread);
    let mut threads = Vec::new();

    for i in 0 .. nthread {
        let t = thread::spawn(move || write_worker(i as u8, write_sz as usize));
        threads.push(t);
    }

    for t in threads {
        t.join().expect("thread failed");
    }

    let valid = verify_file(write_sz as usize);
    if (!valid) {
        println!("File is not valid");
    } else {
        println!("File is valid");
    }
}
