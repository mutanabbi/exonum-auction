mod id;
mod transactions;
mod contracts;
mod schema;
mod errors;

extern crate tokio_core;
extern crate argparse;
#[macro_use]
extern crate exonum;
extern crate hyper;

use exonum::{
    crypto::{PublicKey, SecretKey},
    encoding::serialize::FromHexError,
};
use argparse::{ArgumentParser, Store};


#[derive(Debug)]
enum CliError {
    IncorrectKey(FromHexError),
    IoErr(std::io::Error),
}

impl From<FromHexError> for CliError {
    fn from(err: FromHexError) -> CliError {
        CliError::IncorrectKey(err)
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> CliError {
        CliError::IoErr(err)
    }
}

fn read_keys(id: &String) -> Result<(PublicKey, SecretKey), CliError> {
    use std::fs::read_to_string;
    use exonum::encoding::serialize::FromHex;
    let publ = read_to_string(id.clone() + ".public")?;
    let prvt = read_to_string(id.clone() + ".secret")?;
    Ok((PublicKey::from_hex(publ)?, SecretKey::from_hex(prvt)?))
}

fn gen_and_save_keys(id: &String) -> Result<(PublicKey, SecretKey), CliError> {
    use std::fs::File;
    use std::io::Write;
    let (publ, prvt) = exonum::crypto::gen_keypair();
    let mut public = File::create(id.clone() + ".public")?;
    let mut secret = File::create(id.clone() + ".secret")?;
    public.write(publ.to_hex().as_ref())?;
    secret.write(prvt.to_hex().as_ref())?;
    Ok((publ, prvt))
}


fn main() {
    let mut id: String = String::new();
    let mut url: String = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Exonum light client to deal with auction service");
        ap.refer(&mut id).required()
            .add_option(&["-i", "--identifier"], Store,
            "Text identifier to be able to distinguish between few instances of client");
        ap.refer(&mut url).required()
            .add_option(&["-u", "--url"], Store,
            "Service endpoint");
        ap.parse_args_or_exit();
    }

    let (publ, prvt) = read_keys(&id).or_else(|_| gen_and_save_keys(&id)).unwrap();
    let msg = transactions::TxAnounceLot::new(&publ, "Test lot", 42, &prvt);
    use exonum::encoding::serialize::json::ExonumJson;
    let json = msg.serialize_field().unwrap().to_string();
    println!("{}", json);

    use hyper::{Method, Request, Body, header::HeaderValue, Client, rt::Future, rt::Stream};
    let uri: hyper::Uri = url.parse().unwrap();
    let mut req = Request::new(Body::from(json));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();
    req.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json")
    );
    let client = Client::new();
    let post = client.request(req).and_then(|res| {
        println!("POST: {}", res.status());
        res.into_body().concat2()
    });
    use tokio_core::reactor::Core;
    let mut core = Core::new().unwrap();
    core.run(post).unwrap();

    //let mut mw = exonum::messages::MessageWriter::new(1, 1, 0, );
    //mw.write(msg.anouncer(), msg.anouncer().field_size());
    //mw.write(msg.desc(), msg.desc().field_size());
    //mw.write(msg.price(), msg.price().field_size());
    //mw.sign(&prvt);

/*
    let json = r#"
        {
          "body": {
            "anouncer": "6ce29b2d3ecadc434107ce52c287001c968a1b6eca3e5a1eb62a2419e2924b85",
            "desc": "test lot",
            "price": 42
          },
          "protocol_version": 0,
          "service_id": 1,
          "message_id": 0,
          "signature":"9f684227f1de663775848b3db656bca685e085391e2b00b0e115679fd45443ef58a5abeb555ab3d5f7a3cd27955a2079e5fd486743f36515c8e5bea07992100b"
        }
    "#;
    */
}
