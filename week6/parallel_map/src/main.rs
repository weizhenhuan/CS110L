use crossbeam_channel;
use std::{thread, time};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    output_vec.resize_with(input_vec.len(), Default::default);

    let (s, r) = crossbeam_channel::unbounded();
    let (thread_s, thread_r) = crossbeam_channel::unbounded();
    let mut threads = Vec::new();

    for _ in 0..num_threads {
        let cloned_r = r.clone();
        let thread_s_cloned = thread_s.clone();
        threads.push(thread::spawn(move || {
            while let Ok((i, value)) = cloned_r.recv() {
                let value = f(value);
                thread_s_cloned.send((i, value)).unwrap();
            }
        }))
    }

    let len = input_vec.len();
    for i in 0..len {
        s.send((len - 1 - i, input_vec.pop().unwrap())).unwrap();
    }

    drop(s);
    drop(thread_s);

    while let Ok((i, value)) = thread_r.recv() {
        output_vec[i] = value;
    }

    for t in threads {
        t.join().unwrap();
    }

    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
