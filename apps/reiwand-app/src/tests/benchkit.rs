/// The tree the operations on a workbench are tested on: the editor's first layout of three
/// columns, kept here so the tests do not follow the layouts the editor comes with.
pub fn factory() -> crate::bench::Node {
    use crate::bench::{Dir, Panel::*, group, split};
    let middle = split(
        Dir::Column,
        vec![
            (0.76, group(&[Canvas])),
            (
                0.24,
                group(&[
                    Timeline, Curves, Problems, ViewAs, Inputs, Screens, Sheet, Texts, Cover, Review,
                    Particles, Cost, Scripts, Code, Reference, Console,
                ]),
            ),
        ],
    );
    split(
        Dir::Row,
        vec![
            (0.22, group(&[Layers, Assets, Options, Try, Effects])),
            (0.53, middle),
            (0.25, group(&[Properties])),
        ],
    )
}
