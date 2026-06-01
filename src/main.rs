use tokio;
use colored::*;
use std::fs;
use std::io;
use std::env;
use tokio::net;
use std::net::ToSocketAddrs;




// 1. Read domain from command line
// 2. Read wordlist file into Vec<String>
// 3. Create async tasks for each subdomain
// 4. Perform DNS lookup (tokio::net::lookup_host)
// 5. Print if resolves to IP
// 6. Show total found

#[tokio::main]

async fn main (){
    let domain = "google.com";
    let file_path =  "test.txt";

    // let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
    //     println!("Usage: {} example.com -w path/to/wordlist", args[0]);
    //     return;
    // }

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let subdomains: Vec<&str> = contents.lines().collect();
    println!("Loaded Content: {}", subdomains.len());

    let mut handles = Vec::new();



    for sub in subdomains {
        let full_domain =format!("{}.{}", sub, domain);

        let handle = tokio::spawn(async move {

                match (format!("{}:80", full_domain)).to_socket_addrs() {
                    Ok(addrs) => {
                        for addr in addrs {
                            println!("{} -> {}", full_domain, addr.ip());
                        }
                    }
                    Err(e) => {
                        println!("{} -> {}", full_domain.red(), e);
                    }
                }


        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.await;  // ← Now await each one
        }


}
