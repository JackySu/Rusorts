from timeit import timeit
from typing import Callable
import copy
import random
import rust_sorts


RUNS = 1
TIME_STATS = []
TIME_STATS_N = [
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
]
SORT_FUNCS = {
    "Std sort": rust_sorts.lib.f32_std_sort,
    "Introsort": rust_sorts.lib.f32_introsort,
    "PDQSort": rust_sorts.lib.f32_pdqsort,
    "Quicksort hoare partition": rust_sorts.lib.f32_quicksort_hoare,
    "Quicksort lomuto partition": rust_sorts.lib.f32_quicksort_lomuto,
    "Double Pivot Quicksort": rust_sorts.lib.f32_double_pivot_quicksort,
    "Triple Pivot Quicksort": rust_sorts.lib.f32_triple_pivot_quicksort,
}


def time_sort(sort_name: str, sort_func: Callable[[float], None], to_sort: list[float], stats: list[float]) -> None:
    copied = copy.deepcopy(to_sort)
    time_per_call = timeit(lambda: sort_func(copied, len(copied)), number=RUNS) / RUNS * 1_000_000
    print(f"{sort_name} μs per call: {time_per_call:.2f} μs")
    stats.append(time_per_call)


def run_sorts_with_size(n: int):
    to_sort: list[float] = [random.random() for _ in range(n)]

    stats: list[float] = []

    copied = copy.deepcopy(to_sort)
    python_time_per_call = timeit(lambda: copied.sort(), number=RUNS) / RUNS * 1_000_000
    print(f"Python μs per call: {python_time_per_call:.2f} μs")

    for sort_name, sort_func in SORT_FUNCS.items():
        time_sort(sort_name, sort_func, to_sort, stats)

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

    for i, txt in enumerate(SORT_FUNCS.keys()):
        ax.plot(x, y[i], label=txt, linewidth=1, linestyle='-', marker='o', markersize=3, markeredgecolor='none')

    ax.legend()

    plt.savefig("sorts.png", dpi=300, bbox_inches='tight')
    # plt.show()

    print(" --- done --- ")


if __name__ == '__main__':
    main()
