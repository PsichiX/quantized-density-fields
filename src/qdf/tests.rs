#![cfg(test)]

use super::*;

#[test]
fn test_2d() {
    let mut qdf = QDF::new(2, 9);
    let root = qdf.root();
    assert!(qdf.space_exists(root));
    if let None = qdf.try_get_space(root) {
        assert!(false);
    }
    let space = qdf.space(root).clone();
    assert_eq!(space.id(), root);
    assert_eq!(space.parent(), None);
    assert_eq!(*space.state(), 9);

    qdf.increase_space_density(root).unwrap();
    let space = qdf.space(root).clone();
    assert_eq!(space.subspace().len(), 3);
    let subspace = space.subspace();
    let substate = space.state().subdivide(3);
    assert_eq!(substate, 3);
    assert_eq!(*qdf.space(subspace[0]).state(), substate);
    assert_eq!(*qdf.space(subspace[1]).state(), substate);
    assert_eq!(*qdf.space(subspace[2]).state(), substate);
    assert_eq!(qdf.find_space_neighbors(subspace[0]).unwrap(), vec![subspace[1], subspace[2]]);
    assert_eq!(qdf.find_space_neighbors(subspace[1]).unwrap(), vec![subspace[0], subspace[2]]);
    assert_eq!(qdf.find_space_neighbors(subspace[2]).unwrap(), vec![subspace[0], subspace[1]]);

    let root2 = subspace[0];
    qdf.increase_space_density(root2).unwrap();
    let space2 = qdf.space(root2).clone();
    let subspace2 = space2.subspace();
    let substate2 = space2.state().subdivide(3);
    assert_eq!(substate2, 1);
    assert_eq!(qdf.find_space_neighbors(subspace2[0]).unwrap(), vec![subspace2[1], subspace2[2], subspace[1]]);
    assert_eq!(qdf.find_space_neighbors(subspace2[1]).unwrap(), vec![subspace2[0], subspace2[2], subspace[2]]);
    assert_eq!(qdf.find_space_neighbors(subspace2[2]).unwrap(), vec![subspace2[0], subspace2[1]]);
    assert_eq!(qdf.find_space_neighbors(subspace[0]).unwrap(), vec![]);
    assert_eq!(qdf.find_space_neighbors(subspace[1]).unwrap(), vec![subspace[2], subspace2[0]]);
    assert_eq!(qdf.find_space_neighbors(subspace[2]).unwrap(), vec![subspace[1], subspace2[1]]);

    {
        let mut qdf = QDF::new(2, 9);
        let root = qdf.root();
        qdf.increase_space_density(root).unwrap();
        let space = qdf.space(root).clone();
        assert_eq!(*space.state(), 9);
        for root2 in space.subspace() {
            qdf.increase_space_density(*root2).unwrap();
            let space2 = qdf.space(*root2).clone();
            assert_eq!(*space2.state(), 3);
            for root3 in space2.subspace() {
                let space3 = qdf.space(*root3).clone();
                assert_eq!(*space3.state(), 1);
            }
        }
    }

    qdf.decrease_space_density(root).unwrap();
    assert_eq!(qdf.find_space_neighbors(subspace[0]).unwrap(), vec![subspace[1], subspace[2]]);
    assert_eq!(qdf.find_space_neighbors(subspace[1]).unwrap(), vec![subspace[2], subspace[0]]);
    assert_eq!(qdf.find_space_neighbors(subspace[2]).unwrap(), vec![subspace[1], subspace[0]]);
    qdf.decrease_space_density(root).unwrap();
    let space = qdf.space(root).clone();
    assert_eq!(space.subspace().len(), 0);
}
