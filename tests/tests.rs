extern crate FibHeap;
use FibHeap::fib_heap::FibonacciHeap;

extern crate rand;
use rand::Rng;



#[test]
fn insert_test() -> ()
{
    //let size = 10_000_000;
    let size = 1_000;
    println!("Initialization with size {}", size);

    let mut foo = FibonacciHeap::new();

    let mut rng = rand::thread_rng();
    for _ in 0..size
    {
        let t = rng.gen::<u32>();
        foo.insert(t);
    }
    assert!(foo.get_size() == size);
}


#[test]
fn extract_min_test() -> ()
{
    // let size = 1_000_000;
    let size = 1_000;

    let mut foo = FibonacciHeap::new();
    let mut array: Vec<u32> = Vec::new();

    let mut rng = rand::thread_rng();
    for _ in 0..size
    {
        let t = rng.gen::<u32>();
        foo.insert(t);
        array.push(t);
    }
    array.sort_by(|a,b| b.cmp(a));

    loop
    {
            
        let x = array.pop();
        let y = foo.extract_min();

        if x.is_some() && y.is_some()
        {
            let x_val = x.unwrap();
            let y_val = y.unwrap();
            assert!(x_val == y_val);
        }   
        else
        {
            assert!(x.is_some() == y.is_some());
            break;
        }
    }
}
