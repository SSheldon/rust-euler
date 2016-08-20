/// An iterator that yields the element swaps needed to produce
/// a sequence of all possible permutations for an indexed sequence of
/// elements. Each permutation is only a single swap apart.
///
/// The Steinhaus-Johnson-Trotter algorithm is used.
///
/// Generates even and odd permutations alternately.
///
/// The last generated swap is always (0, 1), and it returns the
/// sequence to its initial order.
struct ElementSwaps {
    sdir: Vec<SizeDirection>,
    /// If `true`, emit the last swap that returns the sequence to initial
    /// state.
    emit_reset: bool,
    swaps_made: usize,
}

impl ElementSwaps {
    /// Creates an `ElementSwaps` iterator for a sequence of `length` elements.
    fn new(length: usize) -> ElementSwaps {
        // Initialize `sdir` with a direction that position should move in
        // (all negative at the beginning) and the `size` of the
        // element (equal to the original index).
        ElementSwaps{
            emit_reset: true,
            sdir: (0..length).map(|i| SizeDirection{ size: i, dir: Direction::Neg }).collect(),
            swaps_made: 0
        }
    }
}

#[derive(Copy, Clone)]
enum Direction { Pos, Neg }

/// An `Index` and `Direction` together.
#[derive(Copy, Clone)]
struct SizeDirection {
    size: usize,
    dir: Direction,
}

impl Iterator for ElementSwaps {
    type Item = (usize, usize);

    // #[inline]
    fn next(&mut self) -> Option<(usize, usize)> {
        fn new_pos_wrapping(i: usize, s: Direction) -> usize {
            i.wrapping_add(match s {
                Direction::Pos => 1,
                Direction::Neg => !0, /* aka -1 */
            })
        }

        fn new_pos(i: usize, s: Direction) -> usize {
            match s {
                Direction::Pos => i + 1,
                Direction::Neg => i - 1,
            }
        }

        // Find the index of the largest mobile element:
        // The direction should point into the vector, and the
        // swap should be with a smaller `size` element.
        let max = self.sdir.iter().cloned().enumerate()
                           .filter(|&(i, sd)|
                                new_pos_wrapping(i, sd.dir) < self.sdir.len() &&
                                self.sdir[new_pos(i, sd.dir)].size < sd.size)
                           .max_by_key(|&(_, sd)| sd.size);
        match max {
            Some((i, sd)) => {
                let j = new_pos(i, sd.dir);
                self.sdir.swap(i, j);

                // Swap the direction of each larger SizeDirection
                for x in &mut self.sdir {
                    if x.size > sd.size {
                        x.dir = match x.dir {
                            Direction::Pos => Direction::Neg,
                            Direction::Neg => Direction::Pos,
                        };
                    }
                }
                self.swaps_made += 1;
                Some((i, j))
            },
            None => if self.emit_reset {
                self.emit_reset = false;
                if self.sdir.len() > 1 {
                    // The last swap
                    self.swaps_made += 1;
                    Some((0, 1))
                } else {
                    // Vector is of the form [] or [x], and the only permutation is itself
                    self.swaps_made += 1;
                    Some((0,0))
                }
            } else { None }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // For a vector of size n, there are exactly n! permutations.
        let n: usize = (2..self.sdir.len() + 1).product();
        (n - self.swaps_made, Some(n - self.swaps_made))
    }
}

pub fn permute<T, F>(v: &mut [T], mut f: F) where F: FnMut(&[T]) {
    let swaps = ElementSwaps::new(v.len());
    for (a, b) in swaps {
        f(v);
        v.swap(a, b);
    }
}
