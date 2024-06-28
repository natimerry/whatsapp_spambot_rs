## Changing csv headers to not break the deserialisation
Either ami retarded ba ei library ta osshikhito, whatever the case, if u need to different headers to deserialize the CSV change the string constants at:
- https://github.com/natimerry/whatsapp_spambot_rs/blob/master/src/csv_parser.rs#L11
- https://github.com/natimerry/whatsapp_spambot_rs/blob/master/src/csv_parser.rs#L13

## Do not file code issues, they will not be fixed and you will be ignored. 
I wrote this in one day and i dont get paid at all to mantain this
Pull requests are welcome
