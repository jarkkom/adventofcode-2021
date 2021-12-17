#!/usr/bin/env julia

function read_input(s)
    bitmap = Dict(
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
    )

    bits = ""
    for (_, c) in enumerate(s)
        bits = bits * bitmap[c]
    end

    bits
end

function parse_varint(bits)
    value = 0
    while true
        has_more = parse(Int, bits[1:1], base = 2)
        value = value * 16| parse(Int, bits[2:5], base = 2)
        bits = SubString(bits, 6)

        if has_more == 0
            break
        end
    end
    value, bits
end    

function read_packet(bits)
    version = parse(Int, bits[1:3], base = 2)
    bits = SubString(bits, 4)

    typeID = parse(Int, bits[1:3], base = 2)
    bits = SubString(bits, 4)

    value = 0
    subPackets = []

    if typeID == 4
        value, bits = parse_varint(bits)
    else
        lengthTypeID = parse(Int, bits[1:1], base = 2)
        bits = SubString(bits, 2)

        if lengthTypeID == 0
            subBitsLen = parse(Int, bits[1:15], base = 2)
            bits = SubString(bits, 16)

            subBits = bits[1:subBitsLen]
            while length(subBits) > 0
                packet, subBits = read_packet(subBits)
                push!(subPackets, packet)                
            end
            bits = SubString(bits, subBitsLen + 1)
        else
            packetCount = parse(Int, bits[1:11], base = 2)
            bits = SubString(bits, 12)

            for i in 1:packetCount
                packet, bits = read_packet(bits)
                push!(subPackets, packet)                
            end
        end
    end

    return Packet(version, typeID, value, subPackets), bits
end

struct Packet
    version::Int
    typeID::Int

    value::Int
    subPackets::Array{Packet}
end

function sum_versions(p)
    sum = p.version
    for sp in p.subPackets
        sum += sum_versions(sp)
    end
    sum
end

function get_value(p)
    if p.typeID == 4
        return p.value
    end

    if p.typeID == 0
        return sum(map(sp -> get_value(sp), p.subPackets))
    end
    if p.typeID == 1
        return prod(map(sp -> get_value(sp), p.subPackets))
    end
    if p.typeID == 2
        return minimum(map(sp -> get_value(sp), p.subPackets))
    end
    if p.typeID == 3
        return maximum(map(sp -> get_value(sp), p.subPackets))
    end
    if p.typeID == 5
        return get_value(p.subPackets[1]) > get_value(p.subPackets[2]) ? 1 : 0
    end
    if p.typeID == 6
        return get_value(p.subPackets[1]) < get_value(p.subPackets[2]) ? 1 : 0
    end
    if p.typeID == 7
        return get_value(p.subPackets[1]) == get_value(p.subPackets[2]) ? 1 : 0
    end
end

input_path = joinpath(@__DIR__, "input.txt")
input = readlines(input_path)

@assert(read_input("D2FE28") == "110100101111111000101000")

@assert(read_packet("110100101111111000101000")[1].version == 6)
@assert(read_packet("110100101111111000101000")[1].typeID == 4)
@assert(read_packet("110100101111111000101000")[1].value == 2021)

@assert(length(read_packet("00111000000000000110111101000101001010010001001000000000")[1].subPackets) == 2)

@assert(sum_versions(read_packet(read_input("8A004A801A8002F478"))[1]) == 16)
@assert(sum_versions(read_packet(read_input("620080001611562C8802118E34"))[1]) == 12)
@assert(sum_versions(read_packet(read_input("C0015000016115A2E0802F182340"))[1]) == 23)
@assert(sum_versions(read_packet(read_input("A0016C880162017C3686B18A3D4780"))[1]) == 31)

println("answer = $(sum_versions(read_packet(read_input(input[1]))[1]))")

@assert(get_value(read_packet(read_input("C200B40A82"))[1]) == 3)
@assert(get_value(read_packet(read_input("04005AC33890"))[1]) == 54)
@assert(get_value(read_packet(read_input("880086C3E88112"))[1]) == 7)
@assert(get_value(read_packet(read_input("CE00C43D881120"))[1]) == 9)
@assert(get_value(read_packet(read_input("D8005AC2A8F0"))[1]) == 1)
@assert(get_value(read_packet(read_input("F600BC2D8F"))[1]) == 0)
@assert(get_value(read_packet(read_input("9C005AC2F8F0"))[1]) == 0)
@assert(get_value(read_packet(read_input("9C0141080250320F1802104A08"))[1]) == 1)

println("answer = $(get_value(read_packet(read_input(input[1]))[1]))")
