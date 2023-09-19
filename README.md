# What it does

export the Rust's
- std lib stable sort function
- what I implemented
  * single-pivot qsort
  * double-pivots qsort
  * triple-pivots qsort
- pdqsort both by std and user

to Python using [PyO3/Maturin](https://github.com/PyO3/maturin)

## ⚠ To fix IEEE-754's NaN, -INF, INF, -0 and 0 issues I implemented a wrapper type `FloatOrd`


# Goal

TODO:
  
  - implement 4, 5, 6 ~ 8 pivots quicksort, optimize and compare their performances
  - add more well-optimzed sorting algorithms into comparisons

# Run

- install Rust🦀 and Python🐍

- `cd` to the directory

- install Maturin, enter virtual env and build release version of sorting package

```
pip install maturin
python3 -m venv .venv
source .venv/bin/activate

maturin develop --release
```

- `cd rust_sorts && python3 sorts.py`

- wait for the results to be stored as `sorts.png`


# Result & what's worth your attention

In my personal test with C w and w/o SSE intrinsics (Modified from concurrent prog assignment, using rust ffi)

> c std qsort time:
> 
>  - 1418117 microseconds, 5095654990 cycles
> 
> c sse qsort time:
> 
>  - 930252 microseconds, 3342629700 cycles
> 
> c sse ssort time:
> 
>  - 1169673 microseconds, 4202931888 cycles
> 
> rust std sort time:
> 
>  - 938202 microseconds, 3371196456 cycles
> 
> rust user implemented introsort time:
> 
>  - 741837 microseconds, 2665609388 cycles
> 
> rust pdq sort time:
> 
>  - 315757 microseconds, 1134595296 cycles
> 
> rust unstable sort (actually PDQSort) time:
> 
>  - 427816 microseconds, 1537250760 cycles
> 
> rust single pivot qsort time:
> 
>  - 910372 microseconds, 3271194432 cycles
> 
> rust double pivot qsort time:
> 
>  - 927852 microseconds, 3334003992 cycles
> 
> rust triple pivot qsort time:
> 
>  - 909853 microseconds, 3269327112 cycles

I re-exported it to python as stated above and get approximately(?) the same results.

<img src="https://github.com/JackySu/Rusorts/blob/master/sorts.png"></img>
