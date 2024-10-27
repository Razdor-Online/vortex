
#[allow(dead_code)]
trait Sender <P> {

    type Writer;
    fn send(&self, packet: P);
}