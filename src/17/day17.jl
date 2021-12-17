#!/usr/bin/env julia

function run_step(x, y, dx, dy)

    x += dx
    y += dy

    dx = dx > 0 ? dx - 1 : dx
    dx = dx < 0 ? dx + 1 : dx

    dy -= 1
    
    (x, y, dx, dy)
end

function run_sim_sample(dx, dy)
    x = y = 0
    max_y = 0

    for step in 1:1000
        (x, y, dx, dy) = run_step(x, y, dx, dy)

        if y > max_y
            max_y = y
        end

        if x in 20:30 && y in -10:-5
            return (true, step, max_y)
        end

        if y < -10
            return (false, step, max_y)
        end
    end
    (false, 1000, max_y)
end

function run_sim_input(dx, dy)
    x = y = 0
    max_y = 0

    for step in 1:1000
        (x, y, dx, dy) = run_step(x, y, dx, dy)

        if y > max_y
            max_y = y
        end

        if x in 34:67 && y in -215:-186
            return (true, step, max_y)
        end

        if y < -215
            return (false, step, max_y)
        end
    end
    (false, 1000, max_y)
end

@assert(run_sim_sample(7, 2) == (true, 7, 3))
@assert(run_sim_sample(6, 3) == (true, 9, 6))
@assert(run_sim_sample(9, 0) == (true, 4, 0))
@assert(run_sim_sample(17, -4) == (false, 3, 0))

best_y = 0
for dx in 1:1000
    for dy in 1:1000
        global best_y
        (hit, _, max_y) = run_sim_input(dx, dy)
        if hit && max_y > best_y
            best_y = max_y
        end
    end
end

println("answer 1 = $(best_y)")

landings = 0
for dx in 1:100
    for dy in -10:10000
        global landings
        (hit, _, max_y) = run_sim_sample(dx, dy)
        if hit
            landings += 1
        end
    end
end
println("sample landings = $(landings)")

landings = 0
for dx in 1:100
    for dy in -215:10000
        global landings
        (hit, _, max_y) = run_sim_input(dx, dy)
        if hit
            landings += 1
        end
    end
end

println("answer 2 = $(landings)")
