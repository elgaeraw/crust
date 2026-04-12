import matplotlib.pyplot as plt  # type: ignore
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
fig = plt.figure()
axes = fig.subplot_mosaic("AC;BC")

fig2, axes2 = plt.subplots(1, 2, figsize=(12, 8))

axes2 = axes2.flatten()

for i, n in enumerate(n_values):
    if i > 2:
        ax = axes2[i % 3]
        for alg, values in data[n].items():
            if n > 9999 and alg in ["Bubble", "Selection", "InsertionDumb"]:
                continue
            x = list(range(1, len(values) + 1))
            ax.scatter(x, values, label=alg)

        ax.set_title(f"n = {n}")
        ax.set_xlabel("Run")
        ax.set_ylabel("Comparisons")
        ax.grid(True, linestyle="--", alpha=0.5)
    else:
        if i == 0:
            tag = "A"
        elif i == 1:
            tag = "B"
        else:
            tag = "C"
        ax = axes[tag]  # type: ignore

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
handles, labels = axes["A"].get_legend_handles_labels()  # type: ignore
handles2, labels2 = axes2[0].get_legend_handles_labels()
fig.legend(handles, labels, loc="upper center", ncol=5)
fig2.legend(handles2, labels2, loc="upper center", ncol=5)

plt.tight_layout(rect=[0, 0, 1, 0.95])
plt.show()
