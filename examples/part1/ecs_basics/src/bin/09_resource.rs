use bevy::prelude::*;

#[derive(Resource, Debug)]
struct GameScore(u32);

#[derive(Resource)]
struct ScorePerEnemy(u32);

fn main() {
    let mut app = App::new();
    app.insert_resource(GameScore(0))
        .insert_resource(ScorePerEnemy(100))
        .add_systems(Update, (add_score, print_score).chain());

    app.update();
    app.update();
}

fn add_score(mut score: ResMut<GameScore>, rule: Res<ScorePerEnemy>) {
    score.0 += rule.0;
}

fn print_score(score: Res<GameScore>) {
    println!("현재 점수: {}", score.0);
}
