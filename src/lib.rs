use std::sync::{Arc,Mutex,};
use std::thread;

fn split_work<T, R>(v: Vec<T>, f: fn(T) -> R) -> Vec<R>
    where
        T: std::marker::Send + 'static + Copy,
        R: std::marker::Send + 'static + std::clone::Clone
{
    let len_v = v.len();
    let threshold: usize = 5;
    if len_v < threshold {
        v.into_iter().map(|x| f(x)).collect()
    } else {
        let result_v: Arc<Mutex<Vec<R>>> = Arc::new(Mutex::new(vec![]));
        for index in v {
            let copy_result_v = Arc::clone(&result_v);
            thread::spawn(move || {
                let mut copy = copy_result_v.lock().unwrap();
                copy.push(f(index));
            }).join().unwrap();
        }
        let mut result: Vec<R> = vec![];
        for _ in 0..len_v {
            let push: Vec<R> = result_v.lock().unwrap().to_vec();
            for i in push {
                result.push(i);
            }
        }
        return result;
    }
}
