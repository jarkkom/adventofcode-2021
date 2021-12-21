#!/usr/bin/env julia

function roll(r)
    r += 1
    if r > 1000
        r -= 1000
    end
    r
end

function play_rounds(pos1, pos2)
    die = 0

    score1 = 0
    score2 = 0

    rolls = 0

    while true
        die = roll(die)
        pos1 += die
        die = roll(die)
        pos1 += die
        die = roll(die)
        pos1 += die

        rolls += 3

        while pos1 > 10
            pos1 -= 10
        end

        score1 += pos1

        if score1 >= 1000
            return (score1, score2, rolls)
        end

        die = roll(die)
        pos2 += die
        die = roll(die)
        pos2 += die
        die = roll(die)
        pos2 += die

        rolls += 3

        while pos2 > 10
            pos2 -= 10
        end

        score2 += pos2

        if score2 >= 1000
            return (score1, score2, rolls)
        end
    end

    return (score1, score2, rolls)
end

(score1, score2, rolls) = play_rounds(4, 8)

@assert(play_rounds(4, 8) == (1000, 745, 993))

(score1, score2, rolls) = play_rounds(3, 7)

println("answer 1 = $(min(score1, score2) * rolls)")
