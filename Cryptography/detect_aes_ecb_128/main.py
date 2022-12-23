with open("ciphertext.txt") as f:
    lines = [line.rstrip('\n') for line in f]

curr_best = ""
curr_best_scr = -1

for line in lines:
    blocks = [line[i:i+16] for i in range(0, len(line), 16)]
    curr_dict = {}
    curr_scr = 0
   
    for b in blocks:
        curr_dict[b] = 0

    for b in blocks:
        curr_dict[b] += 1

    for b in blocks:
        curr_scr += curr_dict[b]

    if curr_scr > curr_best_scr:
        curr_best = line
        curr_best_scr = curr_scr

print("[+] Found result {} with score {}".format(curr_best, curr_best_scr))

