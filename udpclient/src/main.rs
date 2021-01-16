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

use raylib::prelude::*;

fn main() -> std::io::Result<()> {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("MultiHangman")
        .build();

    let res = Resources::new(&mut rl, &thread);

    let mut client = HangmanClient::new("127.0.0.1:22565").unwrap();
    let mut scene: Box<RaylibScene> = Box::new(OpeningScene::new(Arc::clone(&client)));

    let mut current_scene = Scenes::OpeningScene; // TODO change this to the Scenes enum

    while !rl.window_should_close() {
        scene.handle_raylib(&mut rl);
        scene.draw_raylib(&mut rl, &thread, &res); // No next scene, keep drawing

        if scene.has_next_scene() {
            scene = scene.next_scene(Arc::clone(&client));
        }
    }

    println!("Thanks for playing!");
    Ok(())

}


