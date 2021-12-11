#!/usr/bin/env julia

using DelimitedFiles
using Statistics

sample_path = joinpath(@__DIR__, "sample.txt")
input_path = joinpath(@__DIR__, "input.txt")

function read_input(input_path)
    lines = readlines(input_path)

    input = hcat(map(line -> map(n -> parse(Int, n), collect(line)), lines)...)
end

function run_step(octos)
    octos = octos .+ 1

    flash_count = 0
    flashed_octos = Set()
    octos_to_check = Set()

    for oi in CartesianIndices(octos)
        push!(octos_to_check, oi)

        while !isempty(octos_to_check)
            o = pop!(octos_to_check)

            if o in flashed_octos
                continue
            end

            if octos[o] > 9
                push!(flashed_octos, o)
                flash_count += 1

                x, y = Tuple(o)
                for ix in (x-1):(x + 1)
                    for iy in (y-1):(y+1)
                        if ix == 0 && iy == 0
                            continue
                        end

                        ci = CartesianIndex((ix, iy))

                        if checkbounds(Bool, octos, ci)
                            octos[ci] += 1
                            push!(octos_to_check, ci)
                        end
                    end
                end
            end
        end
    end

    for f in flashed_octos
        octos[f] = 0
    end

    return octos, flash_count
end

function run_steps(input)
    tot_flashes = 0
    for s in 1:10000
        input, flashes = run_step(input)
        if flashes == length(input)
            return s
        end
        tot_flashes += flashes
    end
end

sample = read_input(sample_path)
input = read_input(input_path)

@assert(run_steps(sample) == 195)

println("answer = $(run_steps(input))")
