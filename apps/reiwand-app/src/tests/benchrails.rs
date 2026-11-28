use super::{factory, measure};
use crate::bench::{Dir, Panel};
use crate::benchsize::Rail;

#[test]
fn a_side_folded_into_a_rail_takes_its_width_and_its_edge_stays() {
    let mut m = measure();
    let panels = factory().group_of(Panel::Layers).unwrap().panels.clone();
    m.rails = vec![Rail { panels, dir: Dir::Row }];
    m.rail_width = 34.0;
    let (boxes, edges) = m.boxes(&factory(), 1366.0, 768.0);
    let left = boxes.iter().find(|(p, _)| *p == vec![0]).unwrap().1;
    assert_eq!(left.width, 34.0);
    assert!(edges.iter().find(|e| e.path.is_empty() && e.index == 0).unwrap().fixed);
    assert_eq!(m.resize(&factory(), (1366.0, 768.0), &[], 0, 50.0), None);
    let foot = factory().group_of(Panel::Timeline).unwrap().panels.clone();
    m.rails.push(Rail { panels: foot, dir: Dir::Column });
    let (boxes, _) = m.boxes(&factory(), 1366.0, 600.0);
    let path = factory().path_of(Panel::Timeline).unwrap();
    let low = boxes.iter().find(|(p, _)| *p == path).unwrap().1;
    assert_eq!(low.height, 34.0, "the foot did not fold into a strip");
}
