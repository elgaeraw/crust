import matplotlib.pyplot as plt
from collections import defaultdict

# Data structure:
# { n: { algorithm: [values...] } }
data = defaultdict(lambda: defaultdict(list))

with open("values.dat", "r") as f:
    lines = f.readlines()

# skip header
for line in lines[1:]:
    parts = line.split()
    if len(parts) != 3:
        continue

    alg, n, comp = parts
    n = int(n)
    comp = int(comp)

    data[n][alg].append(comp)

# Sort n values
n_values = sorted(data.keys())

# Create subplots (2 rows x 3 columns = 6 graphs)
fig, axes = plt.subplots(2, 2, figsize=(12, 8))

fig2, axes2 = plt.subplots(1, 2, figsize=(12, 8))

axes = axes.flatten()
axes2 = axes2.flatten()

for i, n in enumerate(n_values):
    if i > 3:
        ax = axes2[i % 3]
        for alg, values in data[n].items():
            # if n > 9999 and alg in ["Bubble", "Selection", "InsertionDumb"]:
            #     continue
            x = list(range(1, len(values) + 1))
            plt.scatter(x, values, label=alg)

        ax.xlabel("Run")
        axes2.ylabel("Comparisons")
        axes2.grid(True, linestyle="--", alpha=0.5)
        continue
    ax = axes[i]

    for alg, values in data[n].items():
        # if n > 9999 and alg in ["Bubble", "Selection", "InsertionDumb"]:
        #     continue
        x = list(range(1, len(values) + 1))
        ax.scatter(x, values, label=alg)

    ax.set_title(f"n = {n}")
    ax.set_xlabel("Run")
    ax.set_ylabel("Comparisons")
    ax.grid(True, linestyle="--", alpha=0.5)

# Shared legend (cleaner)
handles, labels = axes[0].get_legend_handles_labels()
fig.legend(handles, labels, loc="upper center", ncol=5)

plt.tight_layout(rect=[0, 0, 1, 0.95])
plt.show()
