use crate::db::*;
use diesel::prelude::*;
use hangmanstructs::*;

#[test]
fn crud_game() {
    let c = conn();

    let ndbu = DbUser::new(&c, "Hello".to_string());
    let ndbg = DbGame::new(&c, GameMode::FastestGuess, "hello".to_string(), 20, ndbu.id);

    println!("Fetching DbGame {:?}", ndbg);
    println!("Fetched DbGame/DbUser is {:?}", DbGame::from(&c, ndbg.id));


    diesel::delete(&ndbg).execute(&c);
    diesel::delete(&ndbu).execute(&c);

    assert_eq!(true, true); // This is a bad test

}
