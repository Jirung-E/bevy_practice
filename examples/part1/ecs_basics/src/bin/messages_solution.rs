use bevy::prelude::*;

#[derive(Message, Debug)]
struct EnemyDefeated {
    score: u32,
    bonus: u32,
}

#[derive(Event)]
struct EnemyDefeatedNow {
    score: u32,
}

#[derive(Resource, Default)]
struct Score(u32);

#[derive(Resource, Default)]
struct SoundCount(u32);

#[derive(Resource, Default)]
struct ImmediateScore(u32);

fn main() {
    let mut app = build_app();
    app.update();
}

fn build_app() -> App {
    let mut app = App::new();
    app.init_resource::<Score>()
        .init_resource::<SoundCount>()
        .init_resource::<ImmediateScore>()
        .add_message::<EnemyDefeated>()
        .add_observer(observe_immediate)
        .add_systems(
            Update,
            (
                defeat_enemies,
                (update_score, play_sound),
                trigger_immediate,
            )
                .chain(),
        );
    app
}

fn defeat_enemies(mut messages: MessageWriter<EnemyDefeated>) {
    messages.write(EnemyDefeated {
        score: 100,
        bonus: 20,
    });
    messages.write(EnemyDefeated {
        score: 150,
        bonus: 30,
    });
}

fn update_score(mut messages: MessageReader<EnemyDefeated>, mut score: ResMut<Score>) {
    for message in messages.read() {
        score.0 += message.score + message.bonus;
    }
}

fn play_sound(mut messages: MessageReader<EnemyDefeated>, mut sound_count: ResMut<SoundCount>) {
    for _ in messages.read() {
        sound_count.0 += 1;
    }
}

fn trigger_immediate(mut commands: Commands) {
    commands.trigger(EnemyDefeatedNow { score: 25 });
}

fn observe_immediate(event: On<EnemyDefeatedNow>, mut score: ResMut<ImmediateScore>) {
    score.0 += event.score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_readers_receive_both_messages_and_observer_runs() {
        let mut app = build_app();
        app.update();

        assert_eq!(app.world().resource::<Score>().0, 300);
        assert_eq!(app.world().resource::<SoundCount>().0, 2);
        assert_eq!(app.world().resource::<ImmediateScore>().0, 25);
    }
}
