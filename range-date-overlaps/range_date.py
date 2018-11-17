from datetime import datetime

def data():
    return [
      (datetime(2018, 1, 1, 20, 0), datetime(2018, 1, 1, 22, 0)),
      (datetime(2018, 1, 1, 22, 0), datetime(2018, 1, 2, 0, 0)),
      (datetime(2018, 1, 1, 20, 30), datetime(2018, 1, 1, 22, 30)),
      (datetime(2018, 1, 1, 21, 59), datetime(2018, 1, 2, 0, 14)),
      (datetime(2018, 1, 2, 0, 5), datetime(2018, 1, 2, 0, 7)),
      (datetime(2018, 1, 1, 23, 30), datetime(2018, 1, 2, 0, 0)),
    ]

def does_overlap(range1, range2):
    if range1[0] < range2[0]:
        return range1[1] > range2[0]
    elif range1[0] > range2[0]:
        return range2[1] > range1[0]

    return True

def search(ranges):
    # store the list of current overlapping dates
    overlaps = []
    # assign a column to an input indexed range
    columns = []

    # sort by starting date first
    for r in ranges:
        overlaps, column = accumulate_overlaps(overlaps, r)
        columns.append(column)

    return columns

def accumulate_overlaps(overlaps, r):
    for i, overlap in enumerate(overlaps):
        if not does_overlap(overlap, r):
            # we replace the overlap if we find a hole
            overlaps[i] = r
            return overlaps, i

    # no hole found, just append and return the initial index
    column = len(overlaps)
    overlaps.append(r)

    return overlaps, column

ranges = sorted(data(), key = lambda x: x[0])
columns = search(ranges)

for r in ranges:
    print("%s–%s" % r)

for columned in columns: #[ranges[column] for column in columns]:
    print(columned)
