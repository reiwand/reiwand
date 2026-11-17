use super::*;

fn offsets(plan: &Plan) -> Vec<i32> {
    plan.shown().iter().map(|s| s.offset).collect()
}

#[test]
fn the_middle_card_is_centred_with_the_rest_on_each_side() {
    let plan = place((1920, 1080), 3, 3, 0.0);
    let middle = plan.shown().iter().find(|s| s.offset == 0).unwrap().rect;
    assert!((middle.x + middle.width / 2.0 - 960.0).abs() < 0.5);
    let left = plan.shown().iter().find(|s| s.offset == -1).unwrap().rect;
    let right = plan.shown().iter().find(|s| s.offset == 1).unwrap().rect;
    assert!(left.x + left.width < middle.x && right.x > middle.x + middle.width);
    assert!(left.height < middle.height);
    assert!(plan.frame.contains(960.0, 497.0) && !plan.frame.contains(left.x, left.y));
    assert_eq!(offsets(&plan).last(), Some(&0), "the middle card is painted last");
}

#[test]
fn cards_wholly_off_the_screen_are_dropped() {
    let plan = place((640, 360), 3, 3, 0.0);
    assert!(plan.shown().iter().all(|s| s.rect.x < 640.0 && s.rect.x + s.rect.width > 0.0));
    assert!(plan.shown().iter().any(|s| s.offset == 0));
    assert!(place((640, 360), 3, 3, 2.0).len < SLOTS);
}

#[test]
fn missing_neighbours_leave_their_places_empty() {
    assert_eq!(offsets(&place((1280, 720), 0, 1, 0.0)).len(), 2);
    assert_eq!(offsets(&place((1280, 720), 0, 0, 0.0)), vec![0]);
}

#[test]
fn sliding_moves_every_card_by_the_same_step() {
    let at = |slide| place((1920, 1080), 3, 3, slide);
    let before = at(0.0).shown().iter().find(|s| s.offset == 1).unwrap().rect;
    let moved = at(0.5).shown().iter().find(|s| s.offset == 1).unwrap().rect;
    assert!(moved.x > before.x);
    let half = at(1.0).shown().iter().find(|s| s.offset == -1).unwrap().rect;
    let middle = at(0.0).shown().iter().find(|s| s.offset == 0).unwrap().rect;
    assert!((half.x - middle.x).abs() < 1.0);
}

#[test]
fn a_point_finds_the_card_under_it() {
    let plan = place((1920, 1080), 3, 3, 0.0);
    assert_eq!(plan.hit(960.0, 497.0), Some(0));
    let right = plan.shown().iter().find(|s| s.offset == 1).unwrap().rect;
    assert_eq!(plan.hit(right.x + 5.0, right.y + 5.0), Some(1));
    assert_eq!(plan.hit(10.0, 10.0), None);
    assert!(plan.inside(960.0, 497.0) && !plan.inside(10.0, 10.0));
}

#[test]
fn small_wheel_deltas_add_up_to_one_step_and_a_turn_back_starts_over() {
    let mut wheel = Wheel::default();
    let steps: Vec<i32> = (0..5).map(|_| wheel.push(notches(0, 10.0))).collect();
    assert_eq!(steps, vec![0, 0, 0, 1, 0]);
    assert_eq!(wheel.push(notches(120, 0.0)), 1);
    assert_eq!(wheel.push(notches(-120, 0.0)), -1);
    wheel.push(0.4);
    assert_eq!(wheel.push(-0.7), 0, "a turn back drops the rest");
    assert_eq!(wheel.push(-0.5), -1);
}

#[test]
fn a_card_samples_its_picture_only_inside_its_own_rectangle() {
    let rect = Rect { x: 100.0, y: 50.0, width: 160.0, height: 90.0 };
    let uv = covering((320, 180), rect, (1000, 500), 1.0);
    let at = |px: f32, py: f32| {
        [uv.offset[0] + uv.scale[0] * px / 1000.0, uv.offset[1] + uv.scale[1] * py / 500.0]
    };
    let [u0, v0] = at(100.0, 50.0);
    let [u1, v1] = at(260.0, 140.0);
    assert!(u0.abs() < 1e-4 && v0.abs() < 1e-4 && (u1 - 1.0).abs() < 1e-4 && (v1 - 1.0).abs() < 1e-4);
    let tall = covering((100, 200), rect, (1000, 500), 1.0);
    assert!((tall.scale[1] * rect.height / 500.0) < 1.0, "a tall picture is cropped, not squeezed");
    let zoomed = covering((320, 180), rect, (1000, 500), 2.0);
    assert!(zoomed.scale[0] < uv.scale[0]);
}
