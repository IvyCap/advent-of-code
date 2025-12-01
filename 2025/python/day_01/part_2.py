



def spin(rots: list):
    dial: int = 50
    zero_count = 0

    for unrot in rots:
        dial_start = dial
        rot = unrot.strip()
        dir = rot[0]
        ticks = int(rot[1:])

        if ticks == 100:
            dial = 0
            zero_count += 1
        elif ticks > 100:
            zero_count += int(ticks / 100)
            ticks = ticks % 100

        match dir.lower():
            case "r": 
                dial += ticks
            case "l":
                dial -= ticks
        

        if dial == 100 or dial == 0:
            dial = 0
            zero_count += 1
        elif dial  > 99:
            dial -= 100
            if dial_start != 0: 
                zero_count += 1
        elif dial < 0:
            dial += 100
            if dial_start != 0: 
                zero_count += 1
        else:
            dial = dial

        # print(ticks)
        print(f"The dial is rotated {rot} to point at {dial}")
        # print(zero_count)

    return zero_count


def main():
    with open('part1_data.txt') as rots:
        count = spin(rots)
        print(count)

if __name__ == "__main__":
    main()
