pub trait PrintableError {
    fn id(&self) -> ID; //may read self because the ID can vary with the kind
    fn msg(&self) -> String;
}

pub type ID = u8;

pub fn output(e: &impl PrintableError) {
    for line in e.msg().lines() {
        eprintln!("[E{:1}] {}", e.id(), line);
    }
}
