#!/usr/bin/env julia

using DelimitedFiles
using Statistics

sample_path = joinpath(@__DIR__, "sample.txt")
input_path = joinpath(@__DIR__, "input.txt")

function read_input(input_path)
    lines = readlines(input_path)

    hcat(map(line -> map(n -> parse(Int, n), collect(line)), lines)...)
end

function get_neighbors(heights, cc)
    x, y = Tuple(cc)

    idxs = [
        #CartesianIndex(x - 1, y - 1),
        CartesianIndex(x, y - 1),
        CartesianIndex(x, y + 1),
        #CartesianIndex(x + 1, y - 1),
        CartesianIndex(x - 1, y),
        CartesianIndex(x + 1, y),
        #CartesianIndex(x - 1, y + 1),
        #CartesianIndex(x + 1, y + 1),
        ]

    within = filter(f -> checkbounds(Bool, heights, f), idxs)

    map(i -> heights[i], within)
end

function get_low_points(heights)
    low_points = []
    for c in CartesianIndices(heights)
        if sum(heights[c] .>= get_neighbors(heights, c)) == 0
            push!(low_points, heights[c])
        end
    end
    low_points
end


sample = read_input(sample_path)
input = read_input(input_path)

@assert(get_low_points(sample) == [1, 0, 5, 5])
@assert(sum(get_low_points(sample) .+= 1) == 15)

println("answer = $(sum(get_low_points(input) .+= 1))")