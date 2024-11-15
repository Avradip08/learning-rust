use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

const LOCKED : bool = true;
const UNLOCKED : bool = false;

struct Mutex<T> { // usually has a data field
    data: UnsafeCell<T>, // shared resource, this is not thread safe by default
    locked: AtomicBool, // by locks
}

unsafe impl<T> Sync for Mutex<T> where T: Send {} // this is a demo of what rust people claim to be fearlessly concurrency.
// Send is for ownership transfer between threads
// Sync is for shared references between threads

impl<T> Mutex<T> { // traits (behaviour) for Mutex
    fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
            locked: AtomicBool::new(UNLOCKED),
        }
    }

    fn tec_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // spin lock
        // while self.locked.load(Ordering::Acquire) == LOCKED {
        //     // keep spinning until lock is acquired

        //     // this is a busy wait loop
        //     // os will interrupt this thread and give time to other threads
        // }
        // not preemtively switch threads here.
        // 1. out of order execution acquire/release only solves out of order execution
        // 2. os can switch threads - still an open problem
        // self.locked.store(LOCKED, Ordering::Release);
        // cannot preemtively switch threads
        //  // preemtively switch threads
        // std::thread::yield_now(); // this is a hint to the os to switch threads
        

        // ABA Problem - very frequent with non blocking data structure especially with CAS.
        
        while self.locked.compare_exchange(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed) != Ok(UNLOCKED) {
            
        }

        let res = f(unsafe { &mut *self.data.get() });
        self.locked.store(UNLOCKED, Ordering :: Relaxed);
        // println!("{:?}",res);
        return res;
    }
}

fn main() {
    println!("Hello, world! This is a mutex tutorial!");

    let mutex = Mutex::new(0);

    let threadfn = ||  {
        for _ in 0..100000 {
            //since mutex does not have a member that does not implement the copy trait
            //we don't have to pass use mutex as a referenece otherwise we implement this like
            //let  _ = &mutex.tec_lock();
            //the above would have been the case if the shared data was string type(does not implement the copy trait) 
            mutex.tec_lock(|data|
                {
                    *data = *data + 1;
                }
            );
        }
    };
    // sp[a]wn threads
    // acquire lock

    std::thread::scope(|scope| {
        for _ in 0..10 {
            scope.spawn(threadfn);
        }
    });

    // join threads
    // for handle in thread_handles {
    //     handle.join().unwrap(); // wait for thread to finish
    //     // println!("Mutex data: {}", mutex.tec_lock(|data| *data));
    // }

    let data = mutex.tec_lock(|data| *data);
    println!("Mutex data: {}", data);
    assert!(data == 10*100000);
}