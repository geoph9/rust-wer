import sys

# Credits: https://martin-thoma.com/word-error-rate-calculation/
def wer(h, r):
    """
    Calculation of WER with Levenshtein distance.

    Works only for iterables up to 254 elements (uint8).
    O(nm) time ans space complexity.

    Parameters
    ----------
    r : list
    h : list

    Returns
    -------
    int

    Examples
    --------
    >>> wer("who is there".split(), "is there".split())
    1
    >>> wer("who is there".split(), "".split())
    3
    >>> wer("".split(), "who is there".split())
    3
    """
    # initialisation
    import numpy
    if len(h) == 0:
        return len(r)

    d = numpy.zeros((len(r) + 1) * (len(h) + 1), dtype=numpy.uint8)
    d = d.reshape((len(r) + 1, len(h) + 1))
    for i in range(len(r) + 1):
        for j in range(len(h) + 1):
            if i == 0:
                d[0][j] = j
            elif j == 0:
                d[i][0] = i

    # computation
    for i in range(1, len(r) + 1):
        for j in range(1, len(h) + 1):
            if r[i - 1] == h[j - 1]:
                d[i][j] = d[i - 1][j - 1]
            else:
                substitution = d[i - 1][j - 1] + 1
                insertion = d[i][j - 1] + 1
                deletion = d[i - 1][j] + 1
                d[i][j] = min(substitution, insertion, deletion)

    return d[len(r)][len(h)]


if __name__ == '__main__':
    f1 = sys.argv[1]
    f2 = sys.argv[2]
    with open(f1, 'r', encoding='utf-8') as f:
        preds = f.readlines()
    with open(f2, 'r', encoding='utf-8') as f:
        truth = f.readlines()
    wers = [wer(l1.split(), l2.split())/len(l2.split()) for l1, l2 in zip(preds, truth)]
    print("FINAL WER:", sum(wers)/len(wers))
