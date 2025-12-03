invalid_total = 0
id_list = []

with open('test_data_small.txt') as file:
     
    for raw_list in file:
        id_list = raw_list.split(",")

    for ids in id_list:
        range_ids = ids.split("-")
        first = int(range_ids[0])
        last = int(range_ids[1]) + 1

        for num in range(first, last):
            length = len(str(num))
            invalid = False

            if length % 2 == 0:
                hlen = int(length / 2) 
                if int(str(num)[:hlen]) == int(str(num)[hlen:]):
                    invalid = True
                else:
                    str_num = str(num)
                    f_digit = int(str_num[0])
                    i = 1
                    j = 1
                    k = 2
                    print(f"MATCH {str_num}")
                    for digit in str_num[1:]:

                        if int(digit) == f_digit:                            
                            length = len(str_num) // i
                            for count in range(length):
                                print(f"i: {i}, j: {j}, k {k}")
                                fset = int(str_num[:i]) 
                                tset = int(str_num[j:k])
                                
                                print(f"FSET: {fset}, TSET: {tset}")
                                if tset == fset:
                                    invalid = True
                                    print("TRUE")
                                else:
                                    invalid = False
                                    print("BREAK")
                                    is_break = True
                                    # break
                                    

                                j = j * (length + 1)
                                k = k * (length + 1)
                                if k > len(str_num) - 1:
                                    break

                            
                        i += 1
                        
            elif length % 2 != 0:
                str_num = str(num)
                f_digit = int(str_num[0])
                same_digit = True
                for digit in str_num:
                    if same_digit == False:
                        # print("BREAK")
                        break
                    elif int(digit) == f_digit:
                        same_digit = True
                        # print(f"NUM: {num}")
                    else:
                        same_digit = False
                        # print("FALSE")
                
                if same_digit == True:
                    invalid = True
                    # print("TRUE")


                        

            # elif invalid == False:
            #     str_num = str(num)

            match invalid:
                case True:
                    invalid_total += num
                    print(f"Invalid Num {num}")




    

print(f"Total: {invalid_total}")