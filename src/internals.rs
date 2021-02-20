pub mod internals {

    use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
    use std::path::Path;
    use text_io::read;

    pub fn lookup(name: &str, mydb: &PickleDb) {
        match mydb.get::<u16>(name) {
            Some(v) => {
                println!("{}'s amount is {}", name, v)
            }
            None => {
                println!("No such entry")
            }
        };
    }

    pub fn remove(name: &str, mydb: &mut PickleDb) {
        match mydb.rem(name) {
            Ok(v) => {
                println!();
                if v {
                    println!("{} has been successfully removed", name)
                } else {
                    println!("Record does not exist")
                }
            }
            Err(e) => {
                println!("Error, failed due to: {}", e)
            }
        };
    }

    pub fn update(mydb: &mut PickleDb) {
        println!();
        println!("Input name followed by amount");
        let name: String = read!();
        let named = capup(&name);
        let amount: u16 = read!();
        println!();
        if mydb.exists(&named[0..named.len()]) {
            mydb.set(&named[0..named.len()], &amount).unwrap();
            println!("{} successfully updated", named)
        } else {
            mydb.set(&named[0..named.len()], &amount).unwrap();
            println!("{} successfully added", named)
        }
    }

    fn iterate(mydb: &PickleDb) {
        for kv in mydb.iter() {
            match kv.get_key() {
                _ => print!("{}", kv.get_key()),
            }
            print!(" ");
        }
        println!();
        println!();
    }

    pub fn bc_handler(val: &str, mydb: &PickleDb) {
        println!();
        println!("These are the available records: ");
        iterate(mydb);
        println!("What name do you want to {}?", val);
    }

    pub fn initdb() -> PickleDb {
        let mydb: PickleDb;
        let root = dirs::config_dir().unwrap();
        let the_way = root.to_str().unwrap();
        let true_way = the_way.to_owned();
        let the_path = true_way + "/.mydb";
        if !Path::new(&the_path[0..the_path.len()]).is_file() {
            mydb = PickleDb::new(
                the_path,
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Cbor,
            )
        } else {
            mydb = PickleDb::load(
                the_path,
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Cbor,
            )
            .unwrap()
        }
        mydb
    }

    pub fn capup(name: &String) -> String {
        let (first, suffix) = name.split_at(1);
        let cap = first.to_uppercase();
        let downfix = suffix.to_lowercase();
        let captup: String = cap + &downfix;

        captup
    }
}
