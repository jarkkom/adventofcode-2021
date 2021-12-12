#!/usr/bin/env julia

using DelimitedFiles

function read_input(input_path)
    g = Dict()
    for l in readlines(input_path)
        src, tgt = split(l, '-')
        if haskey(g, src) 
            push!(g[src], tgt)
        else
            g[src] = [tgt]
        end
        if haskey(g, tgt) 
            push!(g[tgt], src)
        else
            g[tgt] = [src]
        end
    end
    g
end

function depth_first(graph, current, visited, paths)
    push!(visited, current)

    if current == "end"
        push!(paths, visited)
        return
    end

    for c in graph[current]
        if lowercase(c) == c && c in visited 
            continue
        end
        depth_first(graph, c, copy(visited), paths)
    end
    paths
end

sample_path = joinpath(@__DIR__, "sample.txt")
sample2_path = joinpath(@__DIR__, "sample2.txt")
sample3_path = joinpath(@__DIR__, "sample3.txt")
input_path = joinpath(@__DIR__, "input.txt")

sample = read_input(sample_path)
sample2 = read_input(sample2_path)
sample3 = read_input(sample3_path)
input = read_input(input_path)

@assert(length(depth_first(sample, "start", [], [])) == 10)
@assert(length(depth_first(sample2, "start", [], [])) == 19)
@assert(length(depth_first(sample3, "start", [], [])) == 226)

println("answer = $(length(depth_first(input, "start", [], [])))")