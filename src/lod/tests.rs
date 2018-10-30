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
    assert!(level.sublevels().is_empty());

    let lod = LOD::new(2, 1, 4);
    let root = lod.root();
    let level = lod.level(root).clone();
    assert_eq!(level.sublevels().len(), 4);
    for root2 in level.sublevels() {
        let level2 = lod.level(*root2).clone();
        assert_eq!(level2.parent().unwrap(), root);
        assert_eq!(*level2.state(), 1);
        assert!(level2.sublevels().is_empty());
    }

    let mut lod = LOD::new(2, 2, 16);
    let root = lod.root();
    let level = lod.level(root).clone();
    assert_eq!(level.sublevels().len(), 4);
    for root2 in level.sublevels() {
        let level2 = lod.level(*root2).clone();
        assert_eq!(level2.parent().unwrap(), root);
        assert_eq!(*level2.state(), 4);
        for root3 in level2.sublevels() {
            let level3 = lod.level(*root3).clone();
            assert_eq!(level3.parent().unwrap(), *root2);
            assert_eq!(*level3.state(), 1);
            assert!(level3.sublevels().is_empty());
        }
    }
    {
        let root2 = level.sublevels()[0];
        let level2 = lod.level(root2).clone();
        let root3 = level2.sublevels()[0];
        lod.set_level_state(root3, 5).unwrap();
        let level3 = lod.level(root3).clone();
        assert_eq!(*level3.state(), 5)
    }
    assert_eq!(*lod.state(), 20);

    {
        let root2a = level.sublevels()[0];
        let level2a = lod.level(root2a).clone();
        let root3a = level2a.sublevels()[0];
        let root2b = level.sublevels()[3];
        let level2b = lod.level(root2b).clone();
        let root3b = level2b.sublevels()[3];
        assert_eq!(lod.find_path(root2a, root2b).unwrap(), vec![root2a, root2b]);
        assert_eq!(
            lod.find_path(root3a, root3b).unwrap(),
            vec![
                root3a,
                level2a.sublevels()[1],
                level2b.sublevels()[1],
                level2b.sublevels()[0],
                root3b,
            ]
        );
    }
}
