extern crate sfml;

#[macro_use]
extern crate lazy_static;

use std::str;
use udpclient::newgamewizard::NewGameWizardScene;
use udpclient::game::GameScene;
use udpclient::Scene;
use udpclient::Scenes;
use udpclient::hangmanclient::HangmanClient;
use std::sync::Arc;
use std::rc::Rc;
use std::env;
use std::collections::HashMap;
use std::cell::RefCell;

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
    let scenes: HashMap<&str, RefCell<Box<Scene>>> = {
        let mut scenes: HashMap<&str, RefCell<Box<Scene>>> = HashMap::new();

        scenes.insert("NewGameWizardScene", RefCell::new(Box::new(NewGameWizardScene::new(Arc::clone(&client), &font))));
        scenes.insert("GameScene", RefCell::new(Box::new(GameScene::new(Arc::clone(&client), &font))));

        scenes
    };

    let mut start_scene = String::from("NewGameWizardScene"); // TODO change this to the Scenes enum

    /*if let Some(join_id) = env::args().nth(1) {
        let game_id: u64 = join_id.parse().expect("A valid game id is required!");
        client.join_game(game_id);
        sceneindex += 1;
    }*/

    'mainloop: loop {
        {
            let mut scene = scenes.get(start_scene.as_str()).unwrap().borrow_mut();

            while let Some(ev) = window.poll_event() {
                match ev {
                    Event::Closed |
                    Event::KeyPressed { code: Key::Escape, .. }  => break 'mainloop,
                    _ => {}

                }

                scene.handle_event(ev, &mut window);
            }

            scene.draw(&mut window);
        }


        let scene = scenes.get(start_scene.as_str()).unwrap().borrow();
        let (next_scene, next_scene_nm) = scene.next_scene();

        if next_scene {
            start_scene = next_scene_nm;
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
