extern crate sfml;

use std::str;
use udpclient::newgamewizard::NewGameWizardScene;
use udpclient::game::GameScene;
use udpclient::Scene;
use udpclient::hangmanclient::HangmanClient;
use std::sync::Arc;
use std::env;

use sfml::{graphics::*, window::*};

fn main() -> std::io::Result<()> {
    let mut window = RenderWindow::new(
        (800, 600),
        "MultiHangman",
        Style::CLOSE,
        &Default::default(),
    );

    let mut font_path = std::env::current_dir().unwrap();
    font_path.push("Audiowide-Regular.ttf");

    let font_path = font_path.as_path().to_str().unwrap();

    let font = Font::from_file(font_path).unwrap();
    let mut client = HangmanClient::new("127.0.0.1:22565").unwrap(); 

    let mut scenes: Vec<Box<dyn Scene>> = vec![
        Box::new(NewGameWizardScene::new(Arc::clone(&client), &font)),
        Box::new(GameScene::new(Arc::clone(&client), &font))
    ];

    let mut sceneindex = 0;

    if let Some(join_id) = env::args().nth(1) {
        let game_id: u64 = join_id.parse().expect("A valid game id is required!");
        client.join_game(game_id);
        sceneindex += 1;
    }

    'mainloop: loop {
        let scene = &mut scenes[sceneindex];
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed |
                Event::KeyPressed { code: Key::Escape, .. }  => break 'mainloop,
                _ => {}

            }

            scene.handle_event(ev, &mut window);
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
