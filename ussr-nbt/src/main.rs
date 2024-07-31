use std::{
    io::{Cursor, Read},
    time::{Duration, Instant},
};

const N: usize = 10;

fn main() {
    let start: Instant = Instant::now();

    let _buf: Vec<u8> = std::fs::read("../assets/TheAItest_.nbt").unwrap();
    let mut buf: Vec<u8> = Vec::with_capacity(_buf.len());
    flate2::bufread::GzDecoder::new(&_buf[..])
        .read_to_end(&mut buf)
        .unwrap();
    // let buf: Vec<u8> = _buf;

    println!("Done preparing data ({:?})", start.elapsed());

    // let mut reader = Cursor::new(&buf[..]);
    // let result = ussr_nbt::borrow::Nbt::read(&mut reader);
    // println!("{:#?}", result);
    // return;

    let mut ussr_nbt_borrow_times: Vec<Duration> = Vec::with_capacity(N);
    for _ in 0..N {
        let start: Instant = Instant::now();
        let _result = ussr_nbt::borrow::Nbt::read(&mut Cursor::new(&buf[..]));
        ussr_nbt_borrow_times.push(start.elapsed());
    }
    println!(
        "ussr-nbt: {:>15}",
        format!("{:?}", average(ussr_nbt_borrow_times))
    );

    // let mut simdnbt_borrow_times: Vec<Duration> = Vec::with_capacity(N);
    // for _ in 0..N {
    //     let start: Instant = Instant::now();
    //     let _result = simdnbt::borrow::read(&mut Cursor::new(&buf[..]));
    //     simdnbt_borrow_times.push(start.elapsed());
    // }
    // println!(
    //     "simdnbt:  {:>15}",
    //     format!("{:?}", average(simdnbt_borrow_times))
    // );

    // let nbt = simdnbt::borrow::read(&mut Cursor::new(&buf)).unwrap().unwrap();
    // let compound = nbt.as_compound();

    // for tag in compound.iter() {
    //     println!("{:?}", tag);
    // }
}

fn average(times: Vec<Duration>) -> Duration {
    let len = times.len();
    times.into_iter().sum::<Duration>() / len as u32
}
