//TODO: Make this a library so that all the libraries have access to this

pub enum ProtocolError{
    MalformedPayload,
    MalformedHeader,
    InvalidHeaderOption,
    InvalidOptionValue
        
}
pub enum RequestType{
    Heartbeat,
    Join,
    None
}

pub struct Protocol {
    request_type: RequestType,
    body:String,
}

impl Protocol{
    fn header_parser(data: &str) -> Result<Self,ProtocolError>{ // this is a constructor for struct
                                                               // Protocol

        let mut protocol_constructor = Protocol{
            request_type: RequestType::None,
            body:String::from("")
        };

        let Some((header, body)) = data.split_once("\r\n\r\n") else{ 
            return Err(ProtocolError::MalformedPayload);
        };

        protocol_constructor.body = body.to_string();
        let iter = header.split("\r\n");

        for header_line in iter {
            let Some((key,value)) = header_line.split_once(":") else {
                return Err(ProtocolError::MalformedHeader);
            };
            match key{
                "request_type" => {
                    match value{
                        "join" => protocol_constructor.request_type = RequestType::Join,
                        _ => { return Err(ProtocolError::InvalidOptionValue)}
                    }
                },
                _ => {return Err(ProtocolError::InvalidHeaderOption);}
            } 
        }
        Ok(protocol_constructor)
    }
}
