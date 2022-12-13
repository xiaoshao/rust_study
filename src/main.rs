mod arc_demo;
mod thread_scope;

use arc_demo::arc_demo;
fn main() {
    arc_demo();

    thread_scope::thread_scope()
}
