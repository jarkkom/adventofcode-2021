#!/usr/bin/env julia

using DelimitedFiles

function read_input(input_path)
    (template, rest) = Iterators.peel(readlines(input_path))

    rules = Dict()

    for l in rest
        if isempty(l)
            continue
        end

        m = match(r"(\S+) -> (\S+)", l)
        if m !== nothing
            rules[m[1]] = m[2]
        end
    end
    template, rules
end

function process(rules, s)
    res = ""
    for i in 1:length(s) -1
        k = s[i] * s[i + 1]
        if haskey(rules, k)
            res *= s[i]
            res *= rules[k]
        else
            res *= s[i]
        end
    end
    res *= s[length(s)]
end

function find_freqs(s)
    freq = Dict()
    for (_, c) = enumerate(s)
        freq[c] = get(freq, c, 0) + 1
    end

    min(values(freq)...),  max(values(freq)...)
end

sample_path = joinpath(@__DIR__, "sample.txt")
input_path = joinpath(@__DIR__, "input.txt")

sample_template, sample_rules = read_input(sample_path)
input_template, input_rules = read_input(input_path)

s = process(sample_rules, sample_template)
@assert(s == "NCNBCHB")
s = process(sample_rules, s)
@assert(s == "NBCCNBBBCBHCB")
s = process(sample_rules, s)
@assert(s == "NBBBCNCCNBBNBNBBCHBHHBCHB")
s = process(sample_rules, s)
@assert(s == "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB")

for i in 5:10
    global s
    s = process(sample_rules, s)
end

sample_min, sample_max = find_freqs(s)
@assert(sample_min == 161)
@assert(sample_max == 1749)

t = input_template
for i in 1:10
    global t
    t = process(input_rules, t)
end

input_min, input_max = find_freqs(t)

println("answer = $(input_max - input_min)")
