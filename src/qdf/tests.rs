#![cfg(test)]

use super::*;
// use test::Bencher;

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
    let substates = space.state().subdivide(3);
    assert_eq!(substates, vec![3, 3, 3]);
    assert_eq!(*qdf.space(subspace[0]).state(), substates[0]);
    assert_eq!(*qdf.space(subspace[1]).state(), substates[1]);
    assert_eq!(*qdf.space(subspace[2]).state(), substates[2]);
    assert_eq!(
        qdf.find_space_neighbors(subspace[0]).unwrap(),
        vec![subspace[1], subspace[2]]
    );
    assert_eq!(
        qdf.find_space_neighbors(subspace[1]).unwrap(),
        vec![subspace[0], subspace[2]]
    );
    assert_eq!(
        qdf.find_space_neighbors(subspace[2]).unwrap(),
        vec![subspace[0], subspace[1]]
    );

    let root2 = subspace[0];
    qdf.increase_space_density(root2).unwrap();
    let space2 = qdf.space(root2).clone();
    let subspace2 = space2.subspace();
    let substates2 = space2.state().subdivide(3);
    assert_eq!(substates2, vec![1, 1, 1]);
    assert_eq!(
        qdf.find_space_neighbors(subspace2[0]).unwrap(),
        vec![subspace2[1], subspace2[2], subspace[1]]
    );
    assert_eq!(
        qdf.find_space_neighbors(subspace2[1]).unwrap(),
        vec![subspace2[0], subspace2[2], subspace[2]]
    );
    assert_eq!(
        qdf.find_space_neighbors(subspace2[2]).unwrap(),
        vec![subspace2[0], subspace2[1]]
    );
    assert_eq!(qdf.find_space_neighbors(subspace[0]).unwrap(), vec![]);
    assert_eq!(
        qdf.find_space_neighbors(subspace[1]).unwrap(),
        vec![subspace[2], subspace2[0]]
    );
    assert_eq!(
        qdf.find_space_neighbors(subspace[2]).unwrap(),
        vec![subspace[1], subspace2[1]]
    );
    assert_eq!(
        qdf.find_path(subspace2[0], subspace[2]).unwrap(),
        vec![subspace2[0], subspace2[1], subspace[2]]
    );

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

    qdf.set_space_state(root, 27).unwrap();
    assert_eq!(*qdf.space(root).state(), 27);
    assert_eq!(*qdf.space(subspace[0]).state(), 9);
    assert_eq!(*qdf.space(subspace[1]).state(), 9);
    assert_eq!(*qdf.space(subspace[2]).state(), 9);
    assert_eq!(*qdf.space(subspace2[0]).state(), 3);
    assert_eq!(*qdf.space(subspace2[1]).state(), 3);
    assert_eq!(*qdf.space(subspace2[2]).state(), 3);
    qdf.set_space_state(subspace2[0], 6).unwrap();
    assert_eq!(*qdf.space(root).state(), 30);
    assert_eq!(*qdf.space(subspace[0]).state(), 12);
    assert_eq!(*qdf.space(subspace[1]).state(), 9);
    assert_eq!(*qdf.space(subspace[2]).state(), 9);
    assert_eq!(*qdf.space(subspace2[0]).state(), 6);
    assert_eq!(*qdf.space(subspace2[1]).state(), 3);
    assert_eq!(*qdf.space(subspace2[2]).state(), 3);

    qdf.simulation_step::<()>();
    qdf.simulation_step_parallel::<()>();

    qdf.decrease_space_density(root).unwrap();
    assert_eq!(
        qdf.find_space_neighbors(subspace[0]).unwrap(),
        vec![subspace[1], subspace[2]]
    );
    assert_eq!(
        qdf.find_space_neighbors(subspace[1]).unwrap(),
        vec![subspace[2], subspace[0]]
    );
    assert_eq!(
        qdf.find_space_neighbors(subspace[2]).unwrap(),
        vec![subspace[1], subspace[0]]
    );
    assert_eq!(
        qdf.find_path(subspace[0], subspace[2]).unwrap(),
        vec![subspace[0], subspace[2]]
    );
    qdf.decrease_space_density(root).unwrap();
    let space = qdf.space(root).clone();
    assert_eq!(space.subspace().len(), 0);

    {
        let mut qdf = QDF::new(2, 1);
        let root = qdf.root();
        increase_space_density(&mut qdf, root, 10).unwrap();
        for id in &qdf.platonic_spaces {
            let len = qdf.find_space_neighbors(*id).unwrap().len();
            assert!(len > 0 && len <= 3);
        }
    }
}

// #[bench]
// fn bench_simulation_step_level_5_2d(b: &mut Bencher) {
//     let mut qdf = QDF::new(2, 243);
//     let root = qdf.root();
//     increase_space_density(&mut qdf, root, 5).unwrap();
//     b.iter(|| qdf.simulation_step::<()>());
// }
//
// #[bench]
// fn bench_simulation_step_level_10_2d(b: &mut Bencher) {
//     let mut qdf = QDF::new(2, 59049);
//     let root = qdf.root();
//     increase_space_density(&mut qdf, root, 10).unwrap();
//     b.iter(|| qdf.simulation_step::<()>());
// }
//
// #[bench]
// fn bench_simulation_step_parallel_level_5_2d(b: &mut Bencher) {
//     let mut qdf = QDF::new(2, 243);
//     let root = qdf.root();
//     increase_space_density(&mut qdf, root, 5).unwrap();
//     b.iter(|| qdf.simulation_step_parallel::<()>());
// }
//
// #[bench]
// fn bench_simulation_step_parallel_level_10_2d(b: &mut Bencher) {
//     let mut qdf = QDF::new(2, 59049);
//     let root = qdf.root();
//     increase_space_density(&mut qdf, root, 10).unwrap();
//     b.iter(|| qdf.simulation_step_parallel::<()>());
// }

fn increase_space_density(qdf: &mut QDF<i32>, id: ID, depth: usize) -> Result<()> {
    if depth > 0 {
        qdf.increase_space_density(id)?;
        let space = qdf.space(id).clone();
        for id in space.subspace() {
            increase_space_density(qdf, *id, depth - 1)?;
        }
    }
    Ok(())
}
