/// Logging client for nanolog-rs infrastructure
fn main() {
}


/// Log messages are typically 50-100 bytes long,
/// so a flash drive with 250 Mbytes/sec bandwidth
/// can only absorb a few million messages per second.
fn log() {
    println!("Hello, world!");
}


/// The front-end generates two functions per log statement:
/// a record() function that will persist the remaining dynamic log information
/// into an in-memory buffer
/// and a compress() function that will read the contents from the buffers
/// and compress them to an output device.
/// The record() function will replace the original log function in the sources,
/// and compress() will be invoked by the runtime library.
/// These functions allow the runtime to execute highly optimized, inline logic for each log statement.
struct Record {
    bytes: Vec<u8>
}


fn record(message: String) -> Record {
    Record { bytes: message.bytes().collect() }
}

fn compress(record: Record) {}
