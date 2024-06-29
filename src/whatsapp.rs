use std::string::String;
use std::collections::HashSet;
use std::sync::Arc;
use std::thread::{sleep};
use std::time::Duration;
use csv::ReaderBuilder;
use regex::Regex;
use tracing::{debug, error, info, warn};
use crate::csv_parser::Entry;
pub struct Sender{
    form_path: String,
    dump_path: String,
    url: String,
    entries: Vec<Entry>,
    profile: Option<String>,
}

use thirtyfour::prelude::*;
use log::debug as other_debug;
use thirtyfour::action_chain::ActionChain;
use thirtyfour::common::action::KeyAction;
use tokio::time::Sleep;
use thirtyfour::common::keys::Key;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncReadExt;
use std::io::Write;
use clap::builder::Str;
use thirtyfour::Key::Shift;

impl Sender{
    pub fn set_profile(self, profile:String) -> Sender {
        Sender{
            profile: Some(profile),
            ..self
        }
    }




    fn parse_csv(file: std::fs::File) -> Vec<Entry>{
        let re: Regex = Regex::new(r"^(\+?\d{1,2}\s?)?(([0-9]{5}) ?([0-9]{5}))").expect("Unable to compile regex");
        let mut all_entries:Vec<Entry> = vec![];
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
        for result in rdr.deserialize() {
            let mut res: Entry = result.unwrap();
            debug!("{:?}",res);
            let regex_phone_number = re.captures(&res.phone_number);
            match regex_phone_number{
                Some(ref phone_number) => {
                    let fixed_num =  phone_number.get(2).expect("How did we end up here")
                        .as_str().replace(" ","");
                    debug!("Number fixed with regex: {}",fixed_num);
                    res.phone_number=fixed_num.to_string();
                    all_entries.push(res);
                },
                None => {
                    warn!("{} has inputted autistic phone number unparsable by my shitty regex!",res.name);
                }
            }

        }
        all_entries
    }
    pub fn new(form_path:String,dump_path:String,url:String) -> Self{
        let entries = Self::parse_csv(std::fs::File::open(&form_path).unwrap());
        Sender{
            form_path,
            dump_path,
            entries,
            url,
            profile:None,
        }
    }

    async fn type_msg(driver:&WebDriver,entry: &Entry){
        // driver.action_chain().perform().await.unwrap()
        driver.action_chain().send_keys(entry.build_msg().replace('\n',
                                                                  &"\u{e008}\u{e006}".to_string()))
            .perform().await.unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    async fn load_dump(dump_path:&str) -> HashSet<String>{
        let mut str = std::fs::read_to_string(dump_path).expect("Unable to read dump");
        let dump = str.lines().map(|line| line.to_string()).collect::<HashSet<String>>();
        dump
    }

    pub async fn send_msgs(self) -> WebDriverResult<()>{

        let dump = Self::load_dump(&self.dump_path).await;
        info!("{}",format!("Connecting to chromedriver on port: {}",&self.url.clone()));
        let mut caps = DesiredCapabilities::chrome();

        info!("Launching with profile {}",&self.profile.clone().unwrap());
        // caps.set_binary(&self.binary_path.unwrap());

        let profile_path = &format!("--user-data-dir={}",&self.profile.unwrap());
        info!("{}",&profile_path);
        caps.add_arg(profile_path).expect("Failed to set profile");

        let driver = WebDriver::new(&self.url, caps).await?;
        debug!("{:?}",self.entries);

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.dump_path.clone())
            .unwrap();


        for entry in self.entries{

            info!("Sending message to {:?}",entry);
            let url = format!("https://web.whatsapp.com/send?phone=%2B91{}&text&app_absent=0"
                              ,entry.phone_number);

            if dump.contains(&url){
                warn!("Dump e chilo, ignoring {}",&entry.phone_number);
                continue;
            }

            driver.get(url.clone()).await.unwrap();
            let _ = driver.execute("window.onbeforeunload = function() {};",vec![]);
            let elem= driver.query(By::Css("html > body > div:nth-of-type(1) > div > div > div:nth-of-type(2) > div:nth-of-type(4) > div > footer > div:nth-of-type(1) > div > span:nth-of-type(2) > div > div:nth-of-type(2) > div:nth-of-type(1) > div > div:nth-of-type(1)"))
                .first().await;

            match elem{
                Ok(elem) => {
                    driver.action_chain().click_element(&elem).key_down_on_element(&elem,Key::Control).key_down('v')
                        .key_up(Key::Control).key_up('v').perform().await.unwrap();
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    //
                    // driver.action_chain().key_down(Key::Return).key_up(Key::Return).perform().await.unwrap();

                    Self::type_msg(&driver, &entry).await;
                    tokio::time::sleep(Duration::from_secs(1)).await;

                    let _ =driver.action_chain().key_down(Key::Shift).key_down(Key::Enter).key_up(Key::Shift).key_up(Shift).perform().await;

                    info!("Sent message to {:?}",entry.name);
                    let _ = writeln!(file, "{}", url);
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
                Err(e) => {
                    error!("Unable to send a message to {:?} {:?}",entry,e);
                }
            }
        }
        driver.quit().await.unwrap();
        Ok(())
    }
}