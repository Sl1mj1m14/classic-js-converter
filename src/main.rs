use mc_classic;
use mc_classic_js;

mod convert;

fn main () {

    println!("Loading Level");
    let classic = mc_classic::read_level(String::from("test/level.dat")).unwrap();

    println!("Converting Level");
    let js = convert::classic_to_js(classic, 1, 1).unwrap();

    println!("Serializing Level");
    let serialized = mc_classic_js::serialize_data(js);

    println!("Writing Level");
    mc_classic_js::write_data(String::from("test"), serialized, String::from("https://classic.minecraft.net")).unwrap();
}