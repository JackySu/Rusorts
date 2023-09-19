from timeit import timeit
from typing import Callable
import copy

from rust_sorts import f32_std_sort, f32_vec_rng, f32_introsort, f32_pdqsort, f32_quicksort, f32_double_pivot_quicksort, f32_triple_pivot_quicksort


RUNS = 1
TIME_STATS = []
TIME_STATS_FUNCS = [
    f32_std_sort,
    f32_introsort,
    f32_pdqsort,
    f32_quicksort,
    f32_double_pivot_quicksort,
    f32_triple_pivot_quicksort,
]
TIME_STATS_N = [
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
]


def time_sort(sort_func: Callable[[float], None], to_sort: list[float], stats: list[float]) -> None:
    copied = copy.deepcopy(to_sort)
    time_per_call = timeit(lambda: sort_func(copied), number=RUNS) / RUNS * 1_000_000
    print(f"{sort_func.__name__} μs per call: {time_per_call:.2f} μs")
    stats.append(time_per_call)


def run_sorts_with_size(n: int):
    to_sort: list[float] = f32_vec_rng(n)

    stats: list[float] = []

    copied = copy.deepcopy(to_sort)
    python_time_per_call = timeit(lambda: copied.sort(), number=RUNS) / RUNS * 1_000_000
    print(f"Python μs per call: {python_time_per_call:.2f} μs")

    for sort_func in TIME_STATS_FUNCS:
        time_sort(sort_func, to_sort, stats)

    TIME_STATS.append(stats)


def main():
    print("Running sorts on 1,000,000 random floats")

    for n in TIME_STATS_N:
        run_sorts_with_size(n)

    # draw plot of results
    import matplotlib
    matplotlib.use('agg')

    import matplotlib.pyplot as plt
    import numpy as np

    fig, ax = plt.subplots()
    ax.set_title("Sorts")
    ax.set_xlabel("n")
    ax.set_ylabel("μs per call")

    x = np.array(TIME_STATS_N)
    print(x)
    # extract columns
    y = np.array(list(zip(*TIME_STATS)))
    print(y)

    print(" --- plotting --- ")
    plt.ticklabel_format(style='sci', axis='x', scilimits=(0, 0), useMathText=True)
    plt.ticklabel_format(style='sci', axis='y', scilimits=(0, 0), useMathText=True)
    plt.gca().xaxis.get_offset_text().set_visible(True)
    plt.gca().yaxis.get_offset_text().set_visible(True)

    plt.xscale("log")
    plt.xticks(x, [f'$10^{{{i}}}$' for i in range(1, len(TIME_STATS_N) + 1)])
    plt.yscale("log")
    plt.yticks(y[0], [f'$10^{{{i}}}$' for i in range(1, int(np.log10(max(y[0])) + 1))])

    plt.gcf().set_dpi(300)  # Adjust DPI as needed
    plt.gcf().set_size_inches(10, 6)  # Adjust figure size as needed (width, height)

    label_names = [
        "Python",
        "Introsort",
        "PDQSort",
        "Quicksort",
        "Double Pivot Quicksort",
        "Triple Pivot Quicksort",
    ]
    for i, txt in enumerate(label_names):
        ax.plot(x, y[i], label=txt, linewidth=1, linestyle='-', marker='o', markersize=3, markeredgecolor='none')

    ax.legend()

    plt.savefig("sorts.png", dpi=300, bbox_inches='tight')
    # plt.show()

    print(" --- done --- ")


if __name__ == '__main__':
    main()
