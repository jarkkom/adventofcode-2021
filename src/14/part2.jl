#!/usr/bin/env julia

using DelimitedFiles

function read_input(input_path)
    (template, rest) = Iterators.peel(readlines(input_path))

    template_pairs = Dict()

    for i in 1:length(template) -1
        pair = template[i] * template[i + 1]
        if haskey(template_pairs, pair)
            template_pairs[pair] += 1
        else
            template_pairs[pair] = 1
        end
    end

    freq = Dict()
    for (_, c) = enumerate(template)
        cs = "" * c
        freq[cs] = get(freq, cs, 0) + 1
    end
    
    rules = Dict()
    for l in rest
        if isempty(l)
            continue
        end

        m = match(r"(\S+) -> (\S+)", l)
        if m != nothing
            rules[m[1]] = m[2]
        end
    end
    template_pairs, rules, freq
end

function process(rules, pairs, freq)
    new_pairs = Dict()

    for (p, count) in pairs
        p1 = p[1] * rules[p]
        p2 = rules[p] * p[2]

        new_pairs[p1] = get(new_pairs, p1, 0) + count
        new_pairs[p2] = get(new_pairs, p2, 0) + count

        freq[rules[p]] = get(freq, rules[p], 0) + count
    end
    new_pairs, freq
end

sample_path = joinpath(@__DIR__, "sample.txt")
input_path = joinpath(@__DIR__, "input.txt")

sample_template, sample_rules, sample_freq = read_input(sample_path)
input_template, input_rules, input_freq = read_input(input_path)

s = sample_template
for i in 1:40
    global s
    global sample_freq
    s, sample_freq = process(sample_rules, s, sample_freq)
end

sample_min, sample_max = min(values(sample_freq)...), max(values(sample_freq)...)

@assert(sample_min == 3849876073)
@assert(sample_max == 2192039569602)
@assert(sample_max - sample_min == 2188189693529)

t = input_template
for i in 1:40
    global t
    global input_freq
    t, input_freq = process(input_rules, t, input_freq)
end

input_min, input_max = min(values(input_freq)...), max(values(input_freq)...)

println("answer = $(input_max - input_min)")
