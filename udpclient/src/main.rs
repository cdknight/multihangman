extern crate sfml;

#[macro_use]
extern crate lazy_static;

use std::str;
use udpclient::newgamewizard::NewGameWizardScene;
use udpclient::game::GameScene;
use udpclient::Scene;
use udpclient::Scenes;
use udpclient::hangmanclient::HangmanClient;
use udpclient::opening::OpeningScene;
use std::sync::Arc;
use std::rc::Rc;
use std::env;
use std::collections::HashMap;
use std::cell::RefCell;
use udpclient::joingame::JoinGameScene;

use sfml::{graphics::*, window::*};

fn main() -> std::io::Result<()> {
    let mut window = RenderWindow::new(
        (800, 600),
        "MultiHangman",
        Style::CLOSE,
        &Default::default(),
    );

    // let font_send = Rc::clone(&font);
    let font = {
        let mut font_path = std::env::current_dir().unwrap();
        font_path.push("Audiowide-Regular.ttf");
        let font_path = font_path.as_path().to_str().unwrap();

        Font::from_file(font_path).unwrap()
    };


    let mut client = HangmanClient::new("127.0.0.1:22565").unwrap();

    // Massively inefficient (:, but I don't know enough about lifetimes to create these Scenes only when they're necessary.
    // This is inefficient because it creates all the scenes at once instead of only creating them when they're necessary.
    let scenes: HashMap<Scenes, RefCell<Box<Scene>>> = {
        let mut scenes: HashMap<Scenes, RefCell<Box<Scene>>> = HashMap::new();

        scenes.insert(Scenes::OpeningScene, RefCell::new(Box::new(OpeningScene::new(Arc::clone(&client), &font))));
        scenes.insert(Scenes::JoinGameScene, RefCell::new(Box::new(JoinGameScene::new(Arc::clone(&client), &font))));
        scenes.insert(Scenes::NewGameWizardScene, RefCell::new(Box::new(NewGameWizardScene::new(Arc::clone(&client), &font))));
        scenes.insert(Scenes::GameScene, RefCell::new(Box::new(GameScene::new(Arc::clone(&client), &font))));

        scenes
    };

    let mut current_scene = Scenes::OpeningScene; // TODO change this to the Scenes enum

    /*if let Some(join_id) = env::args().nth(1) {
        let game_id: u64 = join_id.parse().expect("A valid game id is required!");
        client.join_game(game_id);
        sceneindex += 1;
    }*/

    'mainloop: loop {
        {
            let mut scene = scenes.get(&current_scene).expect("Couldn't find requested scene").borrow_mut();

            while let Some(ev) = window.poll_event() {
                match ev {
                    Event::Closed |
                    Event::KeyPressed { code: Key::Escape, .. }  => {
                        client.disconnect();
                        break 'mainloop
                    },
                    _ => {}

                }

                scene.handle_event(ev, &mut window);
            }

            scene.draw(&mut window);
        }


        let scene = scenes.get(&current_scene).unwrap().borrow();
        let next_scene = scene.next_scene();

        match next_scene {
            Scenes::None => {},
            _ => { // Any other kind of Scene, which means that the scene has indicated it'd like to switch.
                current_scene = next_scene
            }
        }

        //println!("{:#?}", scene);
    }

    println!("Thanks for playing!");
    Ok(())

}



/*fn new_game(word: String, max_guesses: u16, mode: GameMode) -> std::io::Result<()> {

    {
        let mut socket = UdpSocket::bind("0.0.0.0:0")?;

        let buf = HangmanGame::from(String::from("carver"), 20, User { ip: "127.0.0.1".to_string() }, GameMode::FastestGuess);
        let buf = bincode::serialize(&HangmanEvent::GameCreate(buf)).unwrap();

        println!("{:?}", buf.len());
        socket.send_to(&buf, "127.0.0.1:22565")?;

        let resp = HangmanEventResponse::Err;
        let mut buf = bincode::serialize(&resp).unwrap();
        let (amt, src) = socket.recv_from(buf.as_mut_slice())?;

        let resp: HangmanEventResponse = bincode::deserialize(&buf).unwrap();

        println!("{:?}", resp);
    }

    Ok(())
}*/
