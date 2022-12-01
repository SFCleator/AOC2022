
with open("input.txt") as file:
    vals = file.read().split("\n")

running_val = 0
summed_vals = []
for val in vals:
    if val == '':
        summed_vals.append(running_val)
        running_val = 0
    else:
        running_val += int(val)

print(max(summed_vals))
