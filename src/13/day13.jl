#!/usr/bin/env julia

using DelimitedFiles

function read_input(input_path)
    dots = []
    folds = []

    for l in readlines(input_path)
        if isempty(l)
            continue
        end

        if startswith(l, "fold")
            push!(folds, l)
            continue
        end
        
        x, y = map(n -> parse(Int64, n), split(l, ','))
        push!(dots, (x, y))
    end
    dots, folds
end

function fold_y(dots, fold)
    r = []

    for d in dots
        x, y = d
        if y > fold
            ny = (fold - (y - fold))
            push!(r, (x, ny))
        else
            push!(r, d)
        end
    end
    unique(r)
end

function fold_x(dots, fold)
    r = []

    for d in dots
        x, y = d
        if x > fold
            nx = (fold - (x - fold))
            push!(r, (nx, y))
        else
            push!(r, d)
        end
    end
    unique(r)
end

function do_folds(dots, folds)
    for f in folds
        m = match(r"fold along ([xy])=(\d+)", f)

        fold = parse(Int64, m[2])

        if m[1] == "x"
            dots = fold_x(dots, fold)
        else
            dots = fold_y(dots, fold)
        end
    end
    dots
end

sample_path = joinpath(@__DIR__, "sample.txt")
input_path = joinpath(@__DIR__, "input.txt")

sample_dots, sample_folds = read_input(sample_path)
input_dots, input_folds = read_input(input_path)

@assert(length(fold_y(sample_dots, 7)) == 17)
println("answer 1 = $(length(fold_x(input_dots, 655)))")

folded_input = do_folds(input_dots, input_folds)
for y in 0:10
    for x in 0:80
        if (x, y) in folded_input
            print("#")
        else
            print(".")
        end
    end
    println()
end
