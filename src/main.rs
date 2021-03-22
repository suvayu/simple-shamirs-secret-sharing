use std::env;
use std::fs;
use std::mem;

/// Bytes array character size in bits
static CHARSIZE: usize = 8 * mem::size_of::<u8>(); // bytes -> bits
/// Number of bytes to be encoded as an unsigned integer
static WORDSIZE: usize = mem::size_of::<u128>() / mem::size_of::<u8>(); // 16

/// Encodes a word of bytes into a 128-bit unsigned integer
///
/// Bytes are 8-bits, so the byte word is a slice of length 16 or
/// shorter.  The result is passed back to the caller as a mutable
/// reference, as well as returned by value.  The first is to match
/// the API of the decoding function, and the latter is included for
/// easier debugging.
///
/// # Arguments
///
/// `word` - bytes slice to encode
///
/// `buffer` - mutable reference to pass the result
fn encode_word(word: &[u8], buffer: &mut u128) {
    let mut _tmp: u128 = 0;

    for i in 0..word.len() {
        _tmp = word[i].into();
        *buffer |= _tmp << i * CHARSIZE;
    }
}

/// Decodes a 128-bit unsigned integer into a word of bytes
///
/// Bytes are 8-bits, so the byte word is a slice of length 16 or
/// shorter.  The result is passed back to the caller as a mutable
/// reference.
///
/// # Arguments
///
/// `num` - 128-bit unsigned integer that is to be decoded
///
/// `buffer` - mutable reference to pass the result
fn decode_word(num: u128, buffer: &mut [u8]) {
    let mask: u128 = 0xFF; // 1-byte / 8-bits

    for (i, ele) in buffer.iter_mut().enumerate() {
        *ele = (mask & (num >> i * CHARSIZE)) as u8;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(2, args.len());
    let filename = &args[1];

    let contents: Vec<u8> =
        fs::read(filename).expect(format!("cannot read file: {}", filename).as_str());
    let size = contents.len();
    let mut roundtrip = vec![0u8; size];

    println!("length: {:?}", size);

    let nwords = size / WORDSIZE + 1;
    let mut numeric = vec![0u128; nwords];

    for (i, (input, output)) in contents
        .chunks(16)
        .zip(roundtrip.chunks_mut(16))
        .enumerate()
    {
        encode_word(input, &mut numeric[i]);
        println!("🔒 {:?} -> {:?}", input, numeric[i]);
        decode_word(numeric[i], output);
        println!("🔓 {:?}", output);
    }

    println!("file contents as numbers:\n{:?}", numeric);

    // print out to check
    let text = String::from_utf8(roundtrip).expect("Invalid UTF-8 sequence");
    println!("text:\n{}", text);

    let base: u128 = 2;
    let prime: u128 = base.pow(127) - 1;

    println!("prime: {:?}", prime);
}
