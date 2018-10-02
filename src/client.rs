mod id;
mod transactions;
mod contracts;
mod schema;
mod errors;

extern crate tokio_core;
#[macro_use]
extern crate exonum;
extern crate hyper;
extern crate clap;

use exonum::{
    crypto::{PublicKey, SecretKey},
    encoding::serialize::FromHexError,
};


#[derive(Debug)]
enum CliError {
    IncorrectKey(FromHexError),
    IoErr(std::io::Error),
    CmdErr(String),
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

/*
fn cmd(args: &Vec[String]) -> CliError
{
    assert(! args.is_empty());
    match args[0] {
        "show" => match args[0] {
            "lot" => cmd_show_bid(args[1..]),
            "bid" => cmd_show_bid(args[1..]),
        },
        "add" => match args[0] {
            "lot" => cmd_show_bid(args[1..]),
            "bid" => cmd_show_bid(args[1..]),
        }
    }
}

fn cmd_show(args: &Vec[String]) -> CliError
{
    if args.is_empty() {
        CmdErr("Command \"show\" expects arguments");
    else {
        match args[0] {
            "lot" => cmd_show_bid(args[1..]),
            "bid" => cmd_show_bid(args[1..]),
        }
    }
}

fn cmd_add(args: &Vec[String]) -> CliError
{
    if args.is_empty() {
        CmdErr("Command \"show\" expects arguments");
    }
    else {
        match args[0] {
            "lot" => cmd_show_bid(args[1..]),
            "bid" => cmd_show_bid(args[1..]),
        }
    }
}
*/

fn cmd_add_lot(addr: &str, desc: &str, price: u32, publ: &PublicKey, prvt: &SecretKey) -> Result<(),CliError> {
    let msg = transactions::TxAnounceLot::new(&publ, &desc, price, &prvt);
    use exonum::encoding::serialize::json::ExonumJson;
    let json = msg.serialize_field().unwrap().to_string();
    println!("{}", json);

    use hyper::{Method, Request, Body, header::HeaderValue, Client, rt::Future, rt::Stream};
    let uri: hyper::Uri = ("http://".to_string() + &addr + "/api/services/auction/v1/lot").parse().unwrap();
    println!("{}", uri);
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
    Ok(())
}

// client -i <id> -u <addr> add lot <desc> <price>
// client -i <id> -u <addr> show lot <lot-id> <lot-owner-pub>
// client -i <id> -u <addr> show lot <lot-id> - your own lot @todo
//
// client -i <id> -u <addr> add bid <lot-owner-pub> <lot-id>
//
// client -i <id> -u <addr> show lots
// client -i <id> -u <addr> show lots <lot-owner-pub> @todo
fn main() {
    use clap::{App, Arg, SubCommand};
    let matches = App::new("Exonum light client to deal with auction service")
        .version("1.0.0")
        .author("Ilya Kolesnikovich <ravishankar@mail.ru>")
        .arg(Arg::with_name("identifier")
            .short("i")
            .long("id")
            .value_name("id")
            .help("An identifier to distinguish between few instances of client")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("address")
            .short("a")
            .long("addr")
            .value_name("addr")
            .help("Service endpoint")
            .takes_value(true)
            .required(true)
        )
        .subcommand(
            SubCommand::with_name("show-lot")
                .about("Show information about existing auction lot (including bid info). Auction app uses complex key to identify the lot: {lot-transaction-hash, lot-anouncer-pub}")
                .arg(Arg::with_name("lot-transaction-hash")
                     .help("The lot's blockchain transaction hash to identify it")
                     .required(true)
                     .index(1)
                )
                .arg(Arg::with_name("lot-anouncer-pub")
                    .help("Public key of lot anouncer to identify the lot. Clients publik key by default ")
                    .index(2)
                )
        )
        .subcommand(
            SubCommand::with_name("add-lot")
                .about("Anounce new auction lot")
                .arg(Arg::with_name("description")
                     .help("The lot's full text description")
                     .required(true)
                     .index(1)
                )
                .arg(Arg::with_name("price")
                    .help("Initial price for the lot")
                    .required(true)
                    .index(2)
                )
        )
        .get_matches();

    println!("aaaa");
    let id = matches.value_of("identifier").unwrap().to_string();
    println!("hhhh");
    /// @todo rename addr
    let addr = matches.value_of("address").unwrap();


    println!("ggg");
    let (publ, prvt) = read_keys(&id).or_else(|_| gen_and_save_keys(&id)).unwrap();

    println!("bbb");
    if let Some(sub) = matches.subcommand_matches("show-lot") {
        /*
        cmd_show_lot(
            sub.value_of("lot-transaction-hash").unwrap(),
            sub.value_of("lot-anouncer-pub").or(publ)
        );
        */
    }
    else if let Some(sub) = matches.subcommand_matches("add-lot") {
        cmd_add_lot(
            &addr,
            &sub.value_of("description").unwrap(),
            sub.value_of("price").unwrap().parse().unwrap(),
            &publ,
            &prvt
        );
    }

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
