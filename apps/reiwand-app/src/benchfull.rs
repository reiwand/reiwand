use crate::bench::{Dir, Node, Panel, group, split};
use crate::benchkits::Kit;

fn row(parts: Vec<(f64, Node)>) -> Node {
    split(Dir::Row, parts)
}

fn column(parts: Vec<(f64, Node)>) -> Node {
    split(Dir::Column, parts)
}

/// Each layout with every panel placed: the trees a package page that is not a scene comes with
/// (each without the panels of a scene), and where Window brings back a panel that the layout in
/// use does not hold. The shares are those of a 1920 x 1008 workbench, in pixels.
pub fn full(kit: Kit) -> Node {
    use Panel::*;
    match kit {
        Kit::Scene => row(vec![
            (300.0, group(&[Assets, Scripts, Options, Try, Inputs, Screens, ViewAs, Particles])),
            (
                1240.0,
                column(vec![
                    (840.0, group(&[Canvas])),
                    (
                        168.0,
                        group(&[
                            Timeline, Curves, Problems, Console, Code, Reference, Cost, Sheet, Texts, Cover,
                            Review,
                        ]),
                    ),
                ]),
            ),
            (380.0, column(vec![(500.0, group(&[Properties, Effects])), (508.0, group(&[Layers]))])),
        ]),
        Kit::Particles => row(vec![
            (320.0, column(vec![(560.0, group(&[Particles, Assets, Scripts])), (448.0, group(&[Layers]))])),
            (
                1210.0,
                column(vec![
                    (738.0, group(&[Canvas])),
                    (
                        270.0,
                        group(&[
                            Timeline, Curves, Console, Problems, Code, Reference, Sheet, Texts, Cover, Review,
                        ]),
                    ),
                ]),
            ),
            (
                390.0,
                column(vec![
                    (600.0, group(&[Properties, Effects])),
                    (408.0, group(&[Cost, Try, Options, ViewAs, Inputs, Screens])),
                ]),
            ),
        ]),
        Kit::Scripts => row(vec![
            (280.0, group(&[Scripts, Layers, Assets, Particles])),
            (
                1060.0,
                column(vec![
                    (758.0, group(&[Code, Reference])),
                    (250.0, group(&[Console, Problems, Timeline, Curves, Cost, Sheet, Texts, Cover, Review])),
                ]),
            ),
            (
                580.0,
                column(vec![
                    (392.0, group(&[Canvas])),
                    (616.0, group(&[Inputs, Try, Properties, Effects, Options, ViewAs, Screens])),
                ]),
            ),
        ]),
        Kit::Simulation => row(vec![
            (
                300.0,
                column(vec![
                    (560.0, group(&[Try])),
                    (
                        448.0,
                        group(&[
                            Screens, Console, Problems, Cost, Timeline, Curves, Sheet, Texts, Cover, Review,
                        ]),
                    ),
                ]),
            ),
            (1220.0, group(&[Canvas])),
            (
                400.0,
                column(vec![
                    (420.0, group(&[ViewAs, Properties, Effects])),
                    (588.0, group(&[Inputs, Layers, Assets, Scripts, Code, Reference, Options, Particles])),
                ]),
            ),
        ]),
        Kit::Animation => column(vec![
            (
                540.0,
                row(vec![
                    (300.0, group(&[Layers, Assets, Particles, Scripts])),
                    (1240.0, group(&[Canvas])),
                    (380.0, group(&[Properties, Effects, Options, Try, Cost])),
                ]),
            ),
            (
                468.0,
                row(vec![
                    (1360.0, group(&[Timeline])),
                    (
                        560.0,
                        group(&[
                            Curves, Problems, Console, Code, Reference, ViewAs, Inputs, Screens, Sheet,
                            Texts, Cover, Review,
                        ]),
                    ),
                ]),
            ),
        ]),
        Kit::Wallpaper => row(vec![
            (
                300.0,
                column(vec![
                    (504.0, group(&[Options, Texts])),
                    (
                        504.0,
                        group(&[Assets, Layers, Effects, Properties, Particles, Scripts, Code, Reference]),
                    ),
                ]),
            ),
            (1200.0, group(&[Sheet])),
            (
                460.0,
                column(vec![
                    (200.0, group(&[Cover])),
                    (340.0, group(&[Canvas])),
                    (
                        468.0,
                        group(&[
                            Review, Problems, Console, Cost, Try, Timeline, Curves, ViewAs, Inputs, Screens,
                        ]),
                    ),
                ]),
            ),
        ]),
    }
}
