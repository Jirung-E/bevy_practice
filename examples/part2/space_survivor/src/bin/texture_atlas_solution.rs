use std::time::Duration;

use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct AnimationClip2d {
    start: usize,
    end: usize,
    fps: f32,
    repeat: bool,
}

impl AnimationClip2d {
    fn frame_duration(self) -> Duration {
        Duration::from_secs_f32(1.0 / self.fps)
    }

    fn next_frame(self, current: usize) -> usize {
        if current < self.start || current > self.end {
            return self.start;
        }
        if current < self.end {
            current + 1
        } else if self.repeat {
            self.start
        } else {
            self.end
        }
    }
}

const IDLE_CLIP: AnimationClip2d = AnimationClip2d {
    start: 0,
    end: 3,
    fps: 7.0,
    repeat: true,
};
const WALK_CLIP: AnimationClip2d = AnimationClip2d {
    start: 4,
    end: 7,
    fps: 10.0,
    repeat: true,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Facing {
    Left,
    #[default]
    Right,
}

#[derive(Debug)]
struct AnimationPlayer2d {
    clip: AnimationClip2d,
    frame: usize,
    elapsed: Duration,
}

impl AnimationPlayer2d {
    fn new(clip: AnimationClip2d) -> Self {
        Self {
            clip,
            frame: clip.start,
            elapsed: Duration::ZERO,
        }
    }

    fn set_clip(&mut self, clip: AnimationClip2d) {
        if self.clip != clip {
            self.clip = clip;
            self.frame = clip.start;
            self.elapsed = Duration::ZERO;
        }
    }

    fn tick(&mut self, delta: Duration) {
        self.elapsed += delta;
        let frame_duration = self.clip.frame_duration();
        while self.elapsed >= frame_duration {
            self.elapsed -= frame_duration;
            self.frame = self.clip.next_frame(self.frame);
        }
    }
}

fn update_facing(facing: &mut Facing, direction: Vec2) {
    if direction.x < 0.0 {
        *facing = Facing::Left;
    } else if direction.x > 0.0 {
        *facing = Facing::Right;
    }
}

fn select_clip(player: &mut AnimationPlayer2d, moving: bool) {
    player.set_clip(if moving { WALK_CLIP } else { IDLE_CLIP });
}

fn main() {
    let mut facing = Facing::default();
    let mut animation = AnimationPlayer2d::new(IDLE_CLIP);

    update_facing(&mut facing, Vec2::new(-1.0, 0.0));
    update_facing(&mut facing, Vec2::Y);
    select_clip(&mut animation, true);
    animation.tick(Duration::from_millis(350));

    println!(
        "방향: {facing:?}, 프레임: {}, 프레임 간격: {:?}",
        animation.frame,
        animation.clip.frame_duration()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_input_keeps_last_horizontal_facing() {
        let mut facing = Facing::Right;
        update_facing(&mut facing, Vec2::NEG_X);
        update_facing(&mut facing, Vec2::Y);
        assert_eq!(facing, Facing::Left);
    }

    #[test]
    fn clip_transition_and_repeat_use_clip_data() {
        let mut player = AnimationPlayer2d::new(IDLE_CLIP);
        player.frame = IDLE_CLIP.end;
        player.tick(IDLE_CLIP.frame_duration());
        assert_eq!(player.frame, IDLE_CLIP.start);

        select_clip(&mut player, true);
        assert_eq!(player.clip, WALK_CLIP);
        assert_eq!(player.frame, WALK_CLIP.start);
    }

    #[test]
    fn non_repeating_clip_stops_at_last_frame() {
        let clip = AnimationClip2d {
            start: 8,
            end: 10,
            fps: 12.0,
            repeat: false,
        };
        let mut player = AnimationPlayer2d::new(clip);
        player.tick(Duration::from_secs(1));
        assert_eq!(player.frame, clip.end);
    }
}
