use csv;
use csv::ReaderBuilder;
use tracing::{debug, warn};
use tracing::instrument::WithSubscriber;
use serde::Deserialize;
use regex;
use regex::Regex;

#[derive(Debug, Deserialize)]
pub struct Entry{
    #[serde(rename(deserialize = "Enter your Full Name"))] // change this to suit ur program
    pub name: String,
    #[serde(rename(deserialize = "Enter your WhatsApp phone No "))] // change this to suit ur program
    pub phone_number: String,
}


impl Entry{
    pub fn build_msg(&self) -> String {
        let message = "
Congratulations on becoming a member of Delhi Public School Ruby Park Kolkata's Tech Club! Your passion for technology and dedication to exploring its vast possibilities have brought you here. We're excited to have you on board, and just can't wait to see the amazing contributions you'll bring to our community. Get ready to embark on an incredible journey of innovation and growth.
Click the adjoining link(s) to join us and begin your electrifying journey with the Tech Club.
https://chat.whatsapp.com/DyDTrGkl2jp7IhoOtJU0Bs
For further inquiries, contact us at
Anjishnu Dey - 96574533184
Soham Nandy - 9903737471

`This is an automated message sent through a headless server`
https://github.com/natimerry/whatsapp_spambot_rs
";
        format!("Greetings {} {}",self.name,message)
    }


}



