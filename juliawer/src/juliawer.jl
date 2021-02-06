module juliawer

# Credits: https://martin-thoma.com/word-error-rate-calculation/
function calc_wer(h, r)::Float32
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
    if length(h) == 1 && h[1] == ""
        return 1f0
    end
    d = zeros(UInt8, (length(r) + 1), (length(h) + 1))
    for i = 1:(length(r) + 1)
        for j = 1:(length(h) + 1)
            if i == 1
                d[1, j] = j - 1
            elseif j == 1
                d[i, 1] = i - 1
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
    Float32(d[length(r)+1, length(h)+1]) / Float32(length(r))
end

function run()
    my_transcriptions = []
    original_transcript = []

    open(ARGS[1], "r") do f
        for line in readlines(f)
            push!(my_transcriptions, split(line, " "))
        end
    end

    open(ARGS[2], "r") do f
        for line in readlines(f)
            push!(original_transcript, split(line, " "))
        end
    end

    if length(my_transcriptions) != length(original_transcript)
        println("Invalid lengths")
        exit(1)
    end

    println("Calculating wers...")
    wers = [calc_wer(i[1], i[2]) for i in zip(my_transcriptions, original_transcript)]
    final_wer = sum(wers) / length(wers)
    println("Output WER: ", final_wer)
end

function julia_main()
    try
        run()
    catch
        Base.invokelatest(Base.display_error, Base.catch_stack())
        return 1
    end
    return 0
end

if abspath(PROGRAM_FILE) == @__FILE__
    run()
end

end # module
