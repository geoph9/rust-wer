#=
wer:
- Julia version: 
- Author: geoph
- Date: 2021-02-06
=#

# Credits: https://martin-thoma.com/word-error-rate-calculation/
function wer(h, r)::Float32
    """
    Calculation of WER with Levenshtein distance.

    Works only for iterables up to 254 elements (uint8).
    O(nm) time ans space complexity.

    Parameters
    ----------
    r : Array of true words
    h : Array of predicted words

    Returns
    -------
    float32
    """
    println("H: ", h)
    println("R: ", r)
    if length(h) == 0
        return 1f0
    end

    d = zeros(UInt8, (length(r) + 1), (length(h) + 1))
    for i = 1:(length(r) + 1)
        for j = 1:(length(h) + 1)
            if i == 1
                d[1, j] = j
            elseif j == 1
                d[i, 1] = i
            end
        end
    end
    # computation
    for i = 2:(length(r) + 1)
        for j = 2:(length(h) + 1)
            if r[i - 1] == h[j - 1]
                d[i, j] = d[i - 1, j - 1]
            else
                substitution = d[i - 1, j - 1] + 1
                insertion = d[i, j - 1] + 1
                deletion = d[i - 1, j] + 1
                d[i, j] = min(substitution, insertion, deletion)
            end
        end
    end
    Float32(d[length(r), length(h)]) / Float32(length(r))
end

predictions = ARGS[1]
truth = ARGS[2]

my_transcriptions = []
original_transcript = []

open(predictions, "r") do f
    for line in readlines(f)
        push!(my_transcriptions, split(line, " "))
    end
end

open(predictions, "r") do f
    for line in readlines(f)
        push!(original_transcript, split(line, " "))
    end
end

if length(my_transcriptions) != length(original_transcript)
    println("Invalid lengths")
    exit(1)
end

println("Calculating wers...")
wers = [wer(i[1], i[2]) for i in zip(my_transcriptions, original_transcript)]
println(wers)
final_wer = sum(wers) / length(wers)
println("Output WER: ", final_wer)
