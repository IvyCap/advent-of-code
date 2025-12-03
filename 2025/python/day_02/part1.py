invalid_total = 0
id_list = []

with open('day2_data.txt') as file:
     
    for raw_list in file:
        id_list = raw_list.split(",")

    for ids in id_list:
        range_ids = ids.split("-")
        first = int(range_ids[0])
        last = int(range_ids[1]) + 1

        for num in range(first, last):
            length = len(str(num))

            if length % 2 == 0:
                hlen = int(length / 2) 
                if int(str(num)[:hlen]) == int(str(num)[hlen:]):
                    invalid_total += num
                    print(f"Invalid Num {num}")

    

print(f"Total: {invalid_total}")
    
    
 


