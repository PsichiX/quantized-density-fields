#![cfg(test)]

use super::*;
// use test::Bencher;

#[test]
fn test_2d() {
    let (mut qdf, root) = QDF::new(2, 9);
    assert!(qdf.space_exists(root));
    if let None = qdf.try_get_space(root) {
        assert!(false);
    }
    let space = qdf.space(root).clone();
    assert_eq!(space.id(), root);
    assert_eq!(*space.state(), 9);
    let substates = space.state().subdivide(3);
    assert_eq!(substates, vec![3, 3, 3]);

    let subspace = qdf.increase_space_density(root).unwrap();
    assert_eq!(subspace.len(), 3);
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
    let space2 = qdf.space(root2).clone();
    let substates2 = space2.state().subdivide(3);
    assert_eq!(substates2, vec![1, 1, 1]);
    let subspace2 = qdf.increase_space_density(root2).unwrap();
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
        let (mut qdf, root) = QDF::new(2, 9);
        assert_eq!(*qdf.space(root).state(), 9);
        let subspace = qdf.increase_space_density(root).unwrap();
        for root2 in subspace {
            assert_eq!(*qdf.space(root2).state(), 3);
            let subspace2 = qdf.increase_space_density(root2).unwrap();
            for root3 in subspace2 {
                assert_eq!(*qdf.space(root3).state(), 1);
            }
        }
    }

    qdf.simulation_step::<()>();
    qdf.simulation_step_parallel::<()>();

    let uberspace2 = qdf.decrease_space_density(subspace2[0]).unwrap().unwrap();
    assert_eq!(
        qdf.find_space_neighbors(uberspace2).unwrap(),
        vec![subspace[2], subspace[1]]
    );
    assert_eq!(
        qdf.find_space_neighbors(subspace[1]).unwrap(),
        vec![subspace[2], uberspace2]
    );
    assert_eq!(
        qdf.find_space_neighbors(subspace[2]).unwrap(),
        vec![subspace[1], uberspace2]
    );
    assert_eq!(
        qdf.find_path(uberspace2, subspace[2]).unwrap(),
        vec![uberspace2, subspace[2]]
    );
    let uberspace = qdf.decrease_space_density(uberspace2).unwrap().unwrap();
    assert_eq!(qdf.find_space_neighbors(uberspace).unwrap(), vec![]);

    {
        let (mut qdf, root) = QDF::new(2, 1);
        increase_space_density(&mut qdf, root, 10).unwrap();
        for id in qdf.spaces() {
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
        for id in qdf.increase_space_density(id)? {
            increase_space_density(qdf, id, depth - 1)?;
        }
    }
    Ok(())
}
