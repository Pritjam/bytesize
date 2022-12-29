mod proc;

fn main() {

    let mut processor = proc::build_proc();
    println!("Initial processor state: {}", processor.debug_str());

    for i in 0..10 {
        processor.execute_instruction(i);
        println!("Processor state: {}", processor.debug_str());
    }

    println!("Processor state after execution: {}", processor.debug_str());
}
