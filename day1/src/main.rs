use im::list::List;
use std::sync::Arc;

fn main() {

    let lines = std::io::stdin().lines();

    let l = lines.fold(List::singleton(List::new()), |l: List<List<u32>>, e| {
        match e.expect("Need string").trim() {
            "" => l.cons(List::new()),
            s => {
                let n : u32 = s.parse().expect("integer");
                let newh = l.head().expect("This will be nonempty").cons(n);
                return l.tail().expect("nonempty").cons(newh);
            }
        }
    });

    println!("{:?}", l);

    let list_sum = |l: &std::sync::Arc<List<u32>>| l.iter().map(|v| *v).sum::<u32>();
    let list_sum2 = |l: Arc<List<u32>>| l.iter().map(|v| *v).sum::<u32>();
    let list_cmp = |x: Arc<List<u32>>, y: Arc<List<u32>>| list_sum(&y).cmp(&list_sum(&x));

    let wtf : List<u32> = l.iter().map(list_sum2).collect();

    let max = l.iter().max_by_key(list_sum).expect("max");
    //let max = l.iter().max_by_key(|l| l.iter().map(|v| *v).sum::<u32>()).expect("max");
    let max_sum : u32 = max.iter().map(|v| *v).sum();

    let sorted_max : List<u32> = l.iter().map(list_sum2).collect::<List<u32>>().sort(); //sort().take(3).sum();

    //let boom = sorted_max.iter().map(list_sum2);

    println!("The max is {:?} {}", max, max_sum);

    let s : u32 = sorted_max.reverse().iter().map(|v| *v).take(3).sum();

    println!("Sorted {:?} {}", sorted_max, s);
}
