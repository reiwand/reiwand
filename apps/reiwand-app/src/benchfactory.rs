use crate::bench::{Dir, Node, Panel, group, split};
use crate::benchkits::Kit;

fn row(parts: Vec<(f64, Node)>) -> Node {
    split(Dir::Row, parts)
}

fn column(parts: Vec<(f64, Node)>) -> Node {
    split(Dir::Column, parts)
}

/// The tree a scene's layout comes with: only the panels its kind of work uses, as the design
/// draws them; every other panel starts closed and Window opens it. The shares are those of a
/// 1920 x 1008 workbench, in pixels.
pub fn factory(kit: Kit) -> Node {
    use Panel::*;
    match kit {
        Kit::Scene => row(vec![
            (300.0, group(&[Assets, Scripts])),
            (1240.0, column(vec![(840.0, group(&[Canvas])), (168.0, group(&[Timeline, Problems]))])),
            (380.0, column(vec![(500.0, group(&[Properties, Effects])), (508.0, group(&[Layers]))])),
        ]),
        Kit::Particles => row(vec![
            (320.0, column(vec![(360.0, group(&[Layers])), (648.0, group(&[Particles]))])),
            (1210.0, column(vec![(738.0, group(&[Canvas])), (270.0, group(&[Curves, Console]))])),
            (390.0, column(vec![(600.0, group(&[Properties])), (408.0, group(&[Cost, ViewAs]))])),
        ]),
        Kit::Scripts => row(vec![
            (280.0, group(&[Scripts])),
            (1060.0, column(vec![(758.0, group(&[Code, Reference])), (250.0, group(&[Console, Problems]))])),
            (580.0, column(vec![(392.0, group(&[Canvas])), (616.0, group(&[Inputs, Try]))])),
        ]),
        Kit::Simulation => row(vec![
            (300.0, column(vec![(560.0, group(&[Try])), (448.0, group(&[Screens, Problems]))])),
            (1220.0, group(&[Canvas])),
            (400.0, column(vec![(420.0, group(&[ViewAs])), (588.0, group(&[Inputs]))])),
        ]),
        Kit::Animation => column(vec![
            (
                540.0,
                row(vec![
                    (300.0, group(&[Layers, Assets])),
                    (1240.0, group(&[Canvas])),
                    (380.0, group(&[Properties, Effects])),
                ]),
            ),
            (468.0, row(vec![(1360.0, group(&[Timeline])), (560.0, group(&[Curves]))])),
        ]),
        Kit::Wallpaper => row(vec![
            (300.0, group(&[Options, Texts])),
            (1200.0, group(&[Sheet])),
            (
                460.0,
                column(vec![
                    (200.0, group(&[Cover])),
                    (340.0, group(&[Canvas])),
                    (468.0, group(&[Review, Problems])),
                ]),
            ),
        ]),
    }
}
