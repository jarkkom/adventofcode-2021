#!/usr/bin/env julia

using DelimitedFiles

sample_path = joinpath(@__DIR__, "sample.txt")
input_path = joinpath(@__DIR__, "input.txt")

sample = readlines(sample_path)
input = readlines(input_path)

function get_score(c)
    scores = Dict(
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
    )

    return scores[c]
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
            return get_score(c)
        end
        if c == ']' && last !== '['
            return get_score(c)
        end
        if c == '}' && last !== '{'
            return get_score(c)
        end
        if c == '>' && last !== '<'
            return get_score(c)
        end
    end

    if isempty(stack) 
        return 0
    end

    #get_score(pop!(stack))
    return 0
end



@assert(syntax_error_score("[]") == 0)
@assert(syntax_error_score("{()()()}") == 0)
@assert(syntax_error_score("<([{}])>") == 0)
@assert(syntax_error_score("[<>({}){}[([])<>]]") == 0)
@assert(syntax_error_score("(((((((((())))))))))") == 0)

@assert(sum(map(s -> syntax_error_score(s), sample)) == 26397)

println("answer is $(sum(map(s -> syntax_error_score(s), input)))")