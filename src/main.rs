use std::{collections::HashMap, env, io::{stdout, Write}, thread, time};
use reqwest::{self, Client};

// const URL: &str = "http://localhost:5173/api/test";
const URL: &str = "http://packetfence.pollub/signup";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let email: String;
    let password: String;

    if args.len() == 3 {
        email = args[1].clone();
        password = args[2].clone();
    } else {
        let mut email_buf = String::new();
        let mut password_buf = String::new();
        let _ = stdout().flush();

        println!("Provide your email (e.g. s000000@pollub.edu.pl)");
        std::io::stdin().read_line(&mut email_buf).expect("Bad boy doesnt work :).");
        email = email_buf.lines().collect::<Vec<_>>()[0].to_string();

        println!("Provide your password (e.g. 12345678)");
        std::io::stdin().read_line(&mut password_buf).expect("Bad boy doesnt work :).");
        password = password_buf.lines().collect::<Vec<_>>()[0].to_string()
    }

    println!("Credentials were provided. Trying to connect...");

    let aup = String::from("1");

    let mut params = HashMap::new();

    params.insert("fields[username]", email);
    params.insert("fields[password]", password);
    params.insert("fields[aup]", aup);

    let c = Client::new();

    let mut tries = 1;

    loop {
        if tries > 20 {
            println!("After 20 unsuccessfull tries login was rejected. Seems you have wrong credentials. Check them and try again.");
            return;
        }

        let res_raw = c.post(URL)
            .form(&params)
            .send()
            .await;
            
        if res_raw.is_err() {
            thread::sleep(time::Duration::from_secs(5));
            continue;
        }

        let res = res_raw.unwrap();

        let status = res.status();

        let text= res.text().await.unwrap();

        if text.len() == 0 {
            thread::sleep(time::Duration::from_secs(5));
            continue;
        }

        // Condition is true, when HTML response
        // contains next row
        let is_connected = text.contains("Aktywowanie dostępu do sieci.");

        if !is_connected {
            println!("Unsuccessfull try {}. Trying again...", tries);
            tries += 1;
            thread::sleep(time::Duration::from_secs(5));
            continue;
        }
        
        println!("Succesfully registered: {}", status);
        println!("Internet connection will be available in some time.");
        tries = 0;

        thread::sleep(time::Duration::from_secs(5));
    }
}