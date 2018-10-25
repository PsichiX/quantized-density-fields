#![cfg(test)]

use super::*;

#[test]
fn test_2d() {
    let lod = LOD::new(2, 0, 1);
    let root = lod.root();
    assert!(lod.level_exists(root));
    if let None = lod.try_get_level(root) {
        assert!(false);
    }
    let level = lod.level(root).clone();
    assert_eq!(level.id(), root);
    assert_eq!(level.parent(), None);
    assert_eq!(*level.state(), 1);
    assert!(level.data().is_field());
    assert!(lod.field_exists(level.data().as_field()));

    let lod = LOD::new(2, 1, 4);
    let root = lod.root();
    let level = lod.level(root).clone();
    assert!(level.data().is_sublevels());
    assert_eq!(level.data().as_sublevels().len(), 4);
    for root2 in level.data().as_sublevels() {
        let level2 = lod.level(*root2).clone();
        assert_eq!(level2.parent().unwrap(), root);
        assert_eq!(*level2.state(), 1);
        assert!(level2.data().is_field());
    }

    let mut lod = LOD::new(2, 2, 16);
    let root = lod.root();
    let level = lod.level(root).clone();
    assert!(level.data().is_sublevels());
    assert_eq!(level.data().as_sublevels().len(), 4);
    for root2 in level.data().as_sublevels() {
        let level2 = lod.level(*root2).clone();
        assert_eq!(level2.parent().unwrap(), root);
        assert_eq!(*level2.state(), 4);
        assert!(level2.data().is_sublevels());
        for root3 in level2.data().as_sublevels() {
            let level3 = lod.level(*root3).clone();
            assert_eq!(level3.parent().unwrap(), *root2);
            assert_eq!(*level3.state(), 1);
            assert!(level3.data().is_field());
        }
    }
    {
        let root2 = level.data().as_sublevels()[0];
        let level2 = lod.level(root2).clone();
        let root3 = level2.data().as_sublevels()[0];
        let level3 = lod.level(root3).clone();
        let qdf = lod.field_mut(level3.data().as_field());
        let qdf_root = qdf.root();
        qdf.set_space_state(qdf_root, 5).unwrap();
    }
    lod.recalculate_level_state(root).unwrap();
    assert_eq!(*lod.state(), 20);

    lod.simulation_step::<()>().unwrap();
    lod.simulation_step_parallel::<()>().unwrap();

    {
        let root2a = level.data().as_sublevels()[0];
        let level2a = lod.level(root2a).clone();
        let root3a = level2a.data().as_sublevels()[0];
        let root2b = level.data().as_sublevels()[3];
        let level2b = lod.level(root2b).clone();
        let root3b = level2b.data().as_sublevels()[3];
        assert_eq!(lod.find_path(root2a, root2b).unwrap(), vec![root2a, root2b]);
        assert_eq!(
            lod.find_path(root3a, root3b).unwrap(),
            vec![
                root3a,
                level2a.data().as_sublevels()[1],
                level2b.data().as_sublevels()[1],
                level2b.data().as_sublevels()[0],
                root3b,
            ]
        );
    }
}
