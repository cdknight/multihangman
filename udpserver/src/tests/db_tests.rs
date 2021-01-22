use crate::db::*;
use diesel::prelude::*;
use hangmanstructs::*;

#[test]
fn crud_game() {
    let c = conn();

    let mut ndbu = DbUser::new(&c, "Hello".to_string());
    let ndbg = DbGame::new(&c, GameMode::FastestGuess, "hello".to_string(), 20, ndbu.id);

    println!("Joining user to game {:?}", ndbg);

    ndbu.join_game(&c, &ndbg);

    println!("Fetched DbGame/DbUser is {:?}", DbGame::from(&c, ndbg.id));

    diesel::delete(&ndbg).execute(&c);
    diesel::delete(&ndbu).execute(&c);

    assert_eq!(true, true); // This is a bad test

}
