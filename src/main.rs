use std::env;
use std::fs;
use std::mem;

fn encode_word(word: &[u8]) -> u128 {
    let mut buffer: u128 = 0;
    let mut _tmp: u128 = 0;
    let charsize: usize = 8 * mem::size_of::<u8>(); // bytes -> bits

    for i in 0..word.len() {
        _tmp = word[i].into();
        buffer |= _tmp << i * charsize;
    }
    buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(2, args.len());
    let filename = &args[1];

    let contents: Vec<u8> =
        fs::read(filename).expect(format!("cannot read file: {}", filename).as_str());
    let size = contents.len();

    println!("length: {:?}", size);

    let wordsize: usize = mem::size_of::<u128>() / mem::size_of::<u8>(); // 16
    let nwords = contents.len() / wordsize as usize + 1;
    let mut buffer = vec![0u128; nwords];

    for i in 0..(nwords) {
        buffer[i] = encode_word(if i == nwords - 1 {
            &contents[i * wordsize..]
        } else {
            &contents[i * wordsize..i * wordsize + wordsize]
        });
    }

    println!("buffer: {:?}", buffer);

    let base: u128 = 2;
    let prime: u128 = base.pow(127) - 1;

    println!("prime: {:?}", prime);
}
