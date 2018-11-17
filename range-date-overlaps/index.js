let data = [
  [new Date('2018-01-01T20:00:00'), new Date('2018-01-01T22:00:00')],
  [new Date('2018-01-01T22:00:00'), new Date('2018-01-02T00:00:00')],
  [new Date('2018-01-01T20:30:00'), new Date('2018-01-01T22:30:00')],
  [new Date('2018-01-01T21:59:00'), new Date('2018-01-02T00:14:00')],
  [new Date('2018-01-02T00:05:00'), new Date('2018-01-02T00:07:00')],
  [new Date('2018-01-01T23:30:00'), new Date('2018-01-02T00:00:00')]
];

function does_overlap(range1, range2) {
  if (range1[0] < range2[0]) {
    return range1[1] > range2[0];
  } else if (range2[0] < range1[0]) {
    return range2[1] > range1[0];
  }

  return true;
}

function accumulate_overlaps(overlaps, range) {
  for (let i = 0; i < overlaps.length; i += 1) {
    if (!does_overlap(overlaps[i], range)) {
      overlaps[i] = range;

      return {
        overlaps: overlaps,
        column: i
      };
    }
  }

  let column = overlaps.length;
  overlaps.push(range);

  return {
    overlaps: overlaps,
    column: column
  };
}

function search(ranges) {
  let overlaps = [];
  let columns = [];

  for (let i = 0; i < ranges.length; i += 1) {
    let r = accumulate_overlaps(overlaps, ranges[i]);

    overlaps = r.overlaps; // update overlaps
    columns.push(r.column);
  }

  return columns;
}

let result = search(data.sort());

console.log(result);
