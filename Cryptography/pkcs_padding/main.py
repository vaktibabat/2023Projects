def pad(s, k):
    return s + str(hex((k - len(s)))) * (k - len(s))

print(pad("AAAAA", 8))
