use log::{info, debug, error, trace, warn};
fn main(){
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("This is a INFO message");
    debug!("This is a DEBUG message");
    error!("This is a ERROR message");
    trace!("This is a TRACE message");
    warn!("This is a WARN message");
}
