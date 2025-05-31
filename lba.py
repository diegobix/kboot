SPT = 32

def toChs(lba):
    head = (lba % (SPT * 2)) // SPT
    track = lba // (SPT * 2)
    sector = (lba % SPT + 1)
    print(f"Head: {head}, Track: {track}, Sector: {sector}")

toChs(76)