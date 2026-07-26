use bevy::prelude::*;

#[derive(Message, Debug)]
struct EnemyDefeated {
    score: u32,
}

#[derive(Resource, Default)]
struct Score(u32);

fn main() {
    let mut app = App::new();
    app.init_resource::<Score>()
        .add_message::<EnemyDefeated>()
        .add_systems(Update, (defeat_enemy, update_score).chain());

    app.update();
    app.update();
}

fn defeat_enemy(mut messages: MessageWriter<EnemyDefeated>) {
    messages.write(EnemyDefeated { score: 100 });
}

fn update_score(mut messages: MessageReader<EnemyDefeated>, mut score: ResMut<Score>) {
    for message in messages.read() {
        score.0 += message.score;
        println!("{message:?} 수신, 누적 점수: {}", score.0);
    }
}
