mod borrowing_and_references;
mod slices;
mod stack_heap_allocated;

fn main() {
    stack_heap_allocated::run_examples();
    borrowing_and_references::run_examples();
    slices::run_examples();
}
