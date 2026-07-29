use bevy::prelude::*;

#[derive(Resource, Debug)]
struct GameScore(u32);

#[derive(Resource)]
struct ScorePerEnemy(u32);

#[derive(Resource, Debug, Default)]
struct HighScore(u32);

#[derive(Resource, Debug, Default)]
struct FrameLog(Vec<u32>);

fn main() {
    let mut app = build_app();
    app.update();
    app.update();
}

fn build_app() -> App {
    let mut app = App::new();
    app.insert_resource(GameScore(0))
        .insert_resource(ScorePerEnemy(250))
        .init_resource::<HighScore>()
        .init_resource::<FrameLog>()
        .add_systems(
            Update,
            (add_score, update_high_score, count_frames, print_score).chain(),
        );
    app
}

fn add_score(mut score: ResMut<GameScore>, rule: Res<ScorePerEnemy>) {
    score.0 += rule.0;
}

fn update_high_score(score: Res<GameScore>, mut high_score: ResMut<HighScore>) {
    high_score.0 = high_score.0.max(score.0);
}

fn count_frames(mut frame: Local<u32>, mut log: ResMut<FrameLog>) {
    *frame += 1;
    log.0.push(*frame);
}

fn print_score(score: Res<GameScore>, high_score: Res<HighScore>) {
    println!("현재 점수: {}, 최고 점수: {}", score.0, high_score.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_score_and_system_local_advance_for_each_update() {
        let mut app = build_app();
        app.update();
        app.update();

        assert_eq!(app.world().resource::<GameScore>().0, 500);
        assert_eq!(app.world().resource::<HighScore>().0, 500);
        assert_eq!(app.world().resource::<FrameLog>().0, vec![1, 2]);
    }
}
