extern crate sfml;

use std::net::UdpSocket;
use std::str;
use hangmanstructs::*;
use udpclient::newgamewizard::NewGameWizardScene;
use udpclient::game::GameScene;
use udpclient::Scene;
use udpclient::hangmanclient::HangmanClient;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::cell::RefCell;

use sfml::{graphics::*, window::*};
const font_path: &'static str = "/usr/share/fonts/adobe-source-han-sans/SourceHanSans-Bold.ttc";

fn main() -> std::io::Result<()> {
    let mut window = RenderWindow::new(
        (800, 600),
        "MultiHangman",
        Style::CLOSE,
        &Default::default(),
    );

    let font = Font::from_file(font_path).unwrap();
    let (mut client, sender) = HangmanClient::new("127.0.0.1:22565");

    let mut client = Arc::new(client.unwrap());

    HangmanClient::listen(Arc::clone(&client), sender);


    let mut scenes: Vec<Box<Scene>> = vec![
        Box::new(NewGameWizardScene::new(Arc::clone(&client), &font)),
        Box::new(GameScene::new(Arc::clone(&client), &font))
    ];

    let mut sceneindex = 0;


    'mainloop: loop {
        let scene = &mut scenes[sceneindex];
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed |
                Event::KeyPressed { code: Key::Escape, .. }  => break 'mainloop,
                _ => {}

            }

            scene.handle_event(ev);
        }

        scene.draw(&mut window);

        if scene.next_scene() {
            sceneindex+=1;
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
