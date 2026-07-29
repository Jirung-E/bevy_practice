#[derive(Debug, PartialEq)]
struct Player {
    name: String,
    x: f32,
    speed: f32,
    health: u32,
    state: PlayerState,
}

#[derive(Debug, PartialEq)]
enum PlayerState {
    Idle,
    Moving,
    Dead,
}

fn move_player(player: &mut Player, direction: f32, delta_seconds: f32) {
    if player.health == 0 {
        player.state = PlayerState::Dead;
        return;
    }
    if direction == 0.0 {
        player.state = PlayerState::Idle;
        return;
    }
    player.x += direction * player.speed * delta_seconds;
    player.state = PlayerState::Moving;
}

fn take_damage(player: &mut Player, amount: u32) {
    player.health = player.health.saturating_sub(amount);
    if player.health == 0 {
        player.state = PlayerState::Dead;
    }
}

fn example_player() -> Player {
    Player {
        name: "Ferris".to_owned(),
        x: 0.0,
        speed: 120.0,
        health: 100,
        state: PlayerState::Idle,
    }
}

fn main() {
    let mut player = example_player();
    move_player(&mut player, -1.0, 0.5);
    take_damage(&mut player, 150);
    move_player(&mut player, 1.0, 1.0);
    println!("{}: {player:?}", player.name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_saturates_and_dead_player_cannot_move() {
        let mut player = example_player();
        move_player(&mut player, -1.0, 0.5);
        assert_eq!(player.x, -60.0);

        take_damage(&mut player, 150);
        assert_eq!(player.health, 0);
        assert_eq!(player.state, PlayerState::Dead);

        move_player(&mut player, 1.0, 1.0);
        assert_eq!(player.x, -60.0);
        assert_eq!(player.state, PlayerState::Dead);
    }
}
