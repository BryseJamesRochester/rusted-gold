use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::time::Instant;
use ring::signature;
use ring::signature::{Ed25519KeyPair, KeyPair};
use project_rusted_gold::*;
use hex::{encode, decode};
extern crate project_rusted_gold;



fn gen_and_store_key() {
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    fs::write("./key", pkcs8_bytes.as_ref()).expect("Unable to write file");
}

fn get_key() -> Ed25519KeyPair {
    let data = fs::read("./key").expect("Unable to read file");
    Ed25519KeyPair::from_pkcs8(data.as_ref()).unwrap()
}

fn main() {
    let keypair = get_key();
    let mut bryse = Client::new(String::from("Bryse"), None, Some(keypair));
    let mut vianca = Client::new(String::from("Vianca"), None, None);
    let mut kj = Client::new(String::from("KJ"), None, None);
    let mut grandma = Client::new(String::from("Grandma"), None, None);

    let gen_block = Block{
        balances:BTreeMap::from(
            [(bryse.address(), 20),
                (vianca.address(), 50)]),
        ..Default::default()};

    bryse.set_genesis(gen_block.clone());
    vianca.set_genesis(gen_block.clone());
    kj.set_genesis(gen_block.clone());
    grandma.set_genesis(gen_block.clone());
    let mut block1 = Block::new(
        bryse.address(),
        10,
        BTreeMap::new(),
        &gen_block
    );


    println!("{} paying {} 5 gold and {} 5 gold", bryse.name, kj.name, grandma.name);
    let mut tx = bryse.post_transaction(vec![(kj.address(), 5), (grandma.address(), 5)], None).unwrap();

    tx.sign(&bryse.keypair);

    println!("{} paying {} 10 gold and {} 5 gold", vianca.name, bryse.name, kj.name);
    let mut tx1 = vianca.post_transaction(vec![(bryse.address(), 10), (kj.address(), 5),], None).unwrap();
    tx1.sign(&vianca.keypair);
    println!("{} paying {} 5 gold", bryse.name, grandma.name);
    let mut tx2 = bryse.post_transaction(vec![(grandma.address(), 5)], None).unwrap();
    tx2.sign(&bryse.keypair);
    let tx_clone = tx.clone();
    let tx1_clone = tx1.clone();
    let tx2_clone = tx2.clone();
    block1.add_transaction(tx_clone);
    block1.add_transaction(tx1_clone);
    block1.add_transaction(tx2_clone);
    // println!("block1 contains tx:{:?}, tx1:{:?}, tx2{:?}",
    //          block1.contains(&tx),
    //          block1.contains(&tx1),
    //          block1.contains(&tx2)
    // );
    println!("block1 balance of 'bryse':{}, 'vianca':{}, 'kj':{}, 'grandma':{}",
             block1.balance_of(&bryse.address()),
             block1.balance_of(&vianca.address()),
             block1.balance_of(&kj.address()),
             block1.balance_of(&grandma.address())
    );
    println!("block1 id pre mine {}", encode(&*block1.id()));
    println!("Mining!");
    let start_mine = Instant::now();
    block1.mine();
    println!("block1 id post mine {}, nonce: {}, time: {}", encode(&*block1.id()), block1.proof, start_mine.elapsed().as_secs());






}
