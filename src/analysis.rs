pub mod analysis {

    use pickledb::PickleDb;
    pub fn mean(mydb: &PickleDb, total: u16) -> f64 {
        let wosh = total as f64;
        let mash = mydb.total_keys() as f64;
        wosh / mash
    }

    pub fn sum(mydb: &PickleDb) -> u16 {
        let mut val: u16;
        let mut total: u16 = 0;
        for kv in mydb.iter() {
            val = kv.get_value().unwrap();
            total = val + total;
        }
        total
    }

    pub fn median(mosh: Vec<u16>) {
        if mosh.len() % 2 != 0 {
            println!("Median is = {}", mosh[mosh.len() / 2 - 1]);
        } else if mosh.len() % 2 == 0 {
            let mido = (mosh[mosh.len() / 2] + mosh[mosh.len() / 2 - 1]) / 2;
            println!("Median is = {}", mido);
        }
    }

    pub fn fill_in(mydb: &PickleDb) -> Vec<u16> {
        let mut mosh: Vec<u16> = Vec::new();
        for kv in mydb.iter() {
            mosh.push(kv.get_value().unwrap());
        }
        mosh.sort();
        mosh
    }

    pub fn extreme_vals(mosh: Vec<u16>) {
        println!("Biggest amount is = {}", mosh.iter().max().unwrap());
        println!();
        println!("Smallest amount is = {}", mosh.iter().min().unwrap());
    }
}
