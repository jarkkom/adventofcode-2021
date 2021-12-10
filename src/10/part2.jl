#!/usr/bin/env julia

using DelimitedFiles
using Statistics

sample_path = joinpath(@__DIR__, "sample.txt")
input_path = joinpath(@__DIR__, "input.txt")

sample = readlines(sample_path)
input = readlines(input_path)

function get_score(c)
    scores = Dict(
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
    )

    return Int64(scores[c])
end

function syntax_error_score(s)
    stack = []

    for (_, c) in enumerate(s)
        if c in ['(', '[', '{', '<']
            push!(stack, c)
            continue
        end

        last = pop!(stack)

        if c == ')' && last !== '('
            return missing
        end
        if c == ']' && last !== '['
            return missing
        end
        if c == '}' && last !== '{'
            return missing
        end
        if c == '>' && last !== '<'
            return missing
        end
    end

    if isempty(stack) 
        return missing
    end

    sum = Int64(0)
    foreach(x -> sum = sum * Int64(5) + get_score(x), reverse(stack))

    return sum

end

@assert(syntax_error_score("[({(<(())[]>[[{[]{<()<>>") == 288957)
@assert(syntax_error_score("[(()[<>])]({[<{<<[]>>(") == 5566)
@assert(syntax_error_score("(((({<>}<{<{<>}{[]{[]{}") == 1480781)
@assert(syntax_error_score("{<[[]]>}<{[{[{[]{()[[[]") == 995444)
@assert(syntax_error_score("<{([{{}}[<[[[<>{}]]]>[]]") == 294)

@assert(median(skipmissing(map(s -> syntax_error_score(s), sample))) == 288957)


sums = sort(collect(skipmissing(map(s -> syntax_error_score(s), input))))

println("answer is $(Int64(median(sums)))")