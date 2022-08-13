use ringbuffer::Ringbuffer;

mod ringbuffer;

//#[derive(Clone,Copy)]
#[derive(Default)]
struct Pair {
    l : i32,
    r : i32
}

pub fn bumsti(num: i32) -> i32 {
    let mut buf : [Pair; 10] = Default::default();

//let mut rb = Ringbuffer::new(&mut buf);

let mut v : Vec<Pair> = Vec::new();

//let mut p = Pair {l:0, r:0};
for i in num..num+21 {
    //p.l = i;
    //p.r = i;
    let p = Pair {l:i, r:i};
    v.push(p);
}

let mut sum = 0;
for pp in v {
    sum += pp.l + pp.r;
}

sum

}

fn main() {

    bumsti(17);

}
