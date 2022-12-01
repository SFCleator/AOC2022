# Get the values
with open("input.txt") as file:
    vals = file.read().split("\n")

# Find all the totals
running_val = 0
summed_vals = []
for val in vals:
    if val == "":
        summed_vals.append(running_val)
        running_val = 0
    else:
        running_val += int(val)

print("Max: ", max(summed_vals))
print("Max 3 sum:", sum(sorted(summed_vals)[-3:]))
