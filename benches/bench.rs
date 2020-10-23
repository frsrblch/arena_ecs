use criterion::*;
use gen_id::*;
use rand::prelude::StdRng;
use rand::{seq::SliceRandom, SeedableRng};

criterion_main! {
    generational_index,
}

criterion_group! {
    generational_index,
    read_values_by_generational_index,
    read_values_by_usize,
    read_values_by_u32,
}

fn read_values_by_generational_index(c: &mut Criterion) {
    let read_ids = get_generational_ids::<ReadValue>(READ_LEN);
    let read_from = get_component_values::<ReadValue>(READ_LEN);

    let indices = get_generational_indices(WRITE_LEN, &read_ids);
    let mut write_to = get_component_zeros::<WriteTo>(WRITE_LEN);

    c.bench_function("read by generational index", |b| {
        b.iter(|| {
            write_to
                .iter_mut()
                .into_iter()
                .zip(indices.iter())
                .for_each(|(value, id)| {
                    *value = *read_from.get(id);
                });
        })
    });
}

fn read_values_by_usize(c: &mut Criterion) {
    let read_ids = get_usize_vec(READ_LEN);
    let read_from = get_vec_values(READ_LEN);

    let indices = get_indices(WRITE_LEN, &read_ids);
    let mut write_to = get_f64_zeros(WRITE_LEN);

    c.bench_function("read by usize index", |b| {
        b.iter(|| {
            write_to
                .iter_mut()
                .zip(indices.iter())
                .for_each(|(value, id)| {
                    if let Some(v) = read_from.get(*id) {
                        *value = *v;
                    }
                });
        })
    });
}

fn read_values_by_u32(c: &mut Criterion) {
    let read_ids = get_u32_vec(READ_LEN);
    let read_from = get_vec_values(READ_LEN);

    let indices = get_indices(WRITE_LEN, &read_ids);
    let mut write_to = get_f64_zeros(WRITE_LEN);

    c.bench_function("read by u32 index", |b| {
        b.iter(|| {
            write_to
                .iter_mut()
                .zip(indices.iter())
                .for_each(|(value, id)| {
                    if let Some(v) = read_from.get(*id as usize) {
                        *value = *v;
                    }
                });
        })
    });
}

fn get_generational_ids<A: Arena<Allocator = FixedAllocator<A>>>(len: usize) -> Vec<Id<A>> {
    let mut alloc = FixedAllocator::<A>::default();
    (0..len).into_iter().map(|_| alloc.create()).collect()
}

fn get_usize_vec(len: usize) -> Vec<usize> {
    (0..len).into_iter().collect()
}

fn get_u32_vec(len: usize) -> Vec<u32> {
    (0..len as u32).into_iter().collect()
}

fn get_generational_indices<A: Arena>(len: usize, source: &[Id<A>]) -> Vec<Id<A>> {
    let rng = &mut get_rng();
    (0..len)
        .into_iter()
        .map(|_| *source.choose(rng).unwrap())
        .collect()
}

fn get_indices<T: Copy>(len: usize, source: &[T]) -> Vec<T> {
    let rng = &mut get_rng();
    (0..len)
        .into_iter()
        .map(|_| *source.choose(rng).unwrap())
        .collect()
}

fn get_component_values<A: Arena<Allocator = FixedAllocator<A>>>(len: usize) -> Component<A, f64> {
    let mut alloc = FixedAllocator::default();
    let mut component = Component::default();

    for _ in 0..len {
        let id = alloc.create();
        component.insert(id, <Id<A> as ValidId<A>>::index(&id) as f64);
    }

    component
}

fn get_component_zeros<A: Arena<Allocator = FixedAllocator<A>>>(len: usize) -> Component<A, f64> {
    let mut alloc = FixedAllocator::default();
    let mut component = Component::default();

    for _ in 0..len {
        let id = alloc.create();
        component.insert(id, 0.0);
    }

    component
}

fn get_vec_values(len: usize) -> Vec<f64> {
    (0..len).into_iter().map(|i| i as f64).collect()
}

fn get_f64_zeros(len: usize) -> Vec<f64> {
    (0..len).into_iter().map(|_| 0.0).collect()
}

fn get_rng() -> StdRng {
    let seed = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    StdRng::from_seed(seed)
}

pub struct ReadValue;
fixed_arena!(ReadValue);

pub struct WriteTo;
fixed_arena!(WriteTo);

const READ_LEN: usize = 256;
const WRITE_LEN: usize = 1024;
