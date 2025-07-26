mod credentials_reader;

use credentials_reader::CredentialsReader;

fn main() {
    let credentials_reader = CredentialsReader::new("./credentials/api_keys.xml");

    println!("Hello, world!");
}
