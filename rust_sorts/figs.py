import csv
import numpy as np
import matplotlib.pyplot as plt
import os

filenames = [
    "block_partition_cpu_cycle.csv",
    "block_partition_time_converted.csv",
    "block_partition_cache_miss.csv",
    "block_partition_branch_miss.csv",
    "classical_time_converted.csv",
    "classical_cpu_cycle.csv",
    "classical_cache_misses.csv",
    "classical_branch_misses.csv",
]

units = [
    "CPU Cycles per Element",
    "Time (μs) per Element",
    "Cache Misses per Element",
    "Branch Misses per Element",
    "Time (μs) per Element",
    "CPU Cycles per Element",
    "Cache Misses per Element",
    "Branch Misses per Element",
]

titles = [
    "Classical vs Block Partition CPU Cycles",
    "Classical vs Block Partition Time",
    "Classical vs Block Partition Cache Misses",
    "Classical vs Block Partition Branch Misses",
    "Classical QuickSort Time",
    "Classical QuickSort CPU Cycles",
    "Classical QuickSort Cache Misses",
    "Classical QuickSort Branch Misses",
]


def plot_data(filename: str, title: str, unit: str):
    # Read categories from the CSV file
    categories = set()
    with open(filename, 'r') as file:
        reader = csv.reader((line for i, line in enumerate(file) if i > 0 and not line.startswith('//')), skipinitialspace=True)
        for row in reader:
            if row and row[0]:
                categories.add(row[0])
    data = {category: {'Size': [], 'Mean': [], 'SD': []} for category in categories}

    # Read data from the CSV file, ignoring lines starting with '//'
    with open(filename, 'r') as file:
        reader = csv.DictReader((line for line in file if not line.startswith('//')),
                                skipinitialspace=True)
        for row in reader:
            category = row['Method']
            size = int(row['Size'])
            mean = float(row['Mean'])
            sd = float(row['SD'])
            data[category]['Size'].append(size)
            data[category]['Mean'].append(mean)
            data[category]['SD'].append(sd)

    # Create a palette for colors
    palette = plt.get_cmap('Set1')

    plt.figure(figsize=(15, 9))
    for i, (category, values) in enumerate(data.items()):
        color = palette(i)
        size = np.array(values['Size'])
        avg = np.array(values['Mean']) / size
        std = np.array(values['SD']) / size
        r1 = np.subtract(avg, std)
        r2 = np.add(avg, std)
        plt.plot(values['Size'], avg, label=category, color=color, linewidth=1.0)
        plt.fill_between(values['Size'], r1, r2, color=color, alpha=0.2)

    # Set x and y axis to logarithmic scale
    plt.xscale('log')
    plt.yscale('log')

    plt.xlabel('Size')
    plt.ylabel(unit)
    plt.title(title)
    plt.legend()
    plt.grid(True)

    # Show plot
    # plt.show()
    plt.savefig(title + '.pdf', format='pdf')
    print("Plot saved to", title + '.pdf')


if __name__ == '__main__':
    os.chdir(os.path.dirname(__file__))
    for i in range(len(filenames)):
        plot_data(filenames[i], titles[i], units[i])