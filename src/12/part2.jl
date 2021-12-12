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
    if haskey(visited, current)
        visited[current] += 1
    else
        visited[current] = 1
    end

    visited_more_than_once = 0
    for (v, k) in Tuple(visited)
        if lowercase(v) == v && k > 1 
            visited_more_than_once += 1
        end
    end

    if visited_more_than_once > 1
        return paths
    end

    if current == "end"
        push!(paths, visited)
        return
    end

    for c in graph[current]
        if c == "start"
            continue
        end
        if lowercase(c) == c && haskey(visited, c) && visited[c] > 1 
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

@assert(length(depth_first(sample, "start", Dict(), [])) == 36)

@assert(length(depth_first(sample2, "start", Dict(), [])) == 103)

@assert(length(depth_first(sample3, "start", Dict(), [])) == 3509)

println("answer = $(length(depth_first(input, "start", Dict(), [])))")