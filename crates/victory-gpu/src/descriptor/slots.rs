//! Space for contiguous descriptor tables in a shader-visible heap.

/// Finds the first free run, preferring starts at or after the cursor.
pub(crate) fn free_run(taken: &[bool], cursor: usize, count: usize) -> Option<usize> {
    // A table starting before the cursor can extend past it, but no table
    // can extend past the physical end of the heap.
    let prefix_end = cursor.saturating_add(count.checked_sub(1)?).min(taken.len());
    for (base, slots) in [(cursor, &taken[cursor..]), (0, &taken[..prefix_end])] {
        let mut free = 0;
        for (offset, &used) in slots.iter().enumerate() {
            free = if used { 0 } else { free + 1 };
            if free == count {
                return Some(base + offset + 1 - count);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::free_run;

    #[test]
    fn released_space_can_span_the_cursor() {
        // The first four slots were allocated and then released.
        assert_eq!(free_run(&[false; 8], 4, 6), Some(0));
        // The same physical run with occupied slots on both sides.
        let taken = [true, false, false, false, false, false, false, true];
        assert_eq!(free_run(&taken, 4, 6), Some(1));
        // The physical ends of the heap cannot form a single table.
        assert_eq!(free_run(&[false, false, true, false], 3, 3), None);
    }

    #[test]
    fn matches_all_candidate_starts_in_small_heaps() {
        for size in 1..=8 {
            for mask in 0..1u32 << size {
                let taken: Vec<_> = (0..size).map(|i| mask & (1 << i) != 0).collect();
                for cursor in 0..size {
                    for count in 1..=size + 1 {
                        let expected = (cursor..size).chain(0..cursor).find(|&start| {
                            taken.get(start..start + count).is_some_and(|s| !s.contains(&true))
                        });
                        assert_eq!(
                            free_run(&taken, cursor, count),
                            expected,
                            "taken={taken:?}, cursor={cursor}, count={count}"
                        );
                    }
                }
            }
        }
    }
}
