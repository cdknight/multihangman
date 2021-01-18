use std::str;
use udpclient::newgamewizard::NewGameWizardScene;
use udpclient::game::GameScene;
use udpclient::Scenes;
use udpclient::hangmanclient::HangmanClient;
use udpclient::opening::OpeningScene;
use std::sync::Arc;
use std::rc::Rc;
use std::env;
use std::collections::HashMap;
use std::cell::RefCell;
use udpclient::joingame::JoinGameScene;
use udpclient::RaylibScene;
use udpclient::resources::Resources;
use udpclient::connect::ConnectScene;

use raylib::prelude::*;

fn main() -> std::io::Result<()> {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("MultiHangman")
        .build();

    let res = Resources::new(&mut rl, &thread);

    // let client =  HangmanClient::new("127.0.0.1:22565").unwrap();
    let mut scene: Box<RaylibScene> = Box::new(ConnectScene::new());

    while !rl.window_should_close() {
        scene.handle_raylib(&mut rl);
        scene.draw_raylib(&mut rl, &thread, &res); // No next scene, keep drawing

        if scene.has_next_scene() {
            scene = scene.next_scene(); // Box doesn't like moving to itself so we can't just consume self. Always make sure self is a reference
        }
    }

    println!("Thanks for playing!");
    Ok(())

}


