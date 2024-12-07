import sys

VAL = {
    "=": -2,
    "-": -1,
    "0": 0,
    "1": 1,
    "2": 2
}

INVERT = {
    "=": -2,
    "-": -1,
    "0": 0,
    "1": 1,
    "2": 2
}

def convertFromSnaf(line):
    base = 1
    result = 0
    for c in line[::-1]:
        result += base*VAL[c]
        base *= 5
    return result

def convert_base5_reversed(number):
    result = []
    while number > 0:
        v = number % 5
        result.append(v)
        number //= 5
    return result  

def convert_base5_to_snaf(base_5):
    carry = 0
    result = []
    for v in base_5:
        test = v + carry
        carry = test // 5
        test = test % 5
        match test:
            case 0 | 1 | 2:
                result.append(str(test))
            case 3:
                carry += 1
                result.append("=")
            case 4:
                carry += 1
                result.append("-")
    if carry > 0:
        result.append(str(carry))
    return "".join(result[::-1])

def test():
    print(convert_base5_to_snaf([3, 4]))

def main():
    nums = [line for line in sys.stdin.read().split("\n")]
    real_nums = [convertFromSnaf(num) for num in nums]
    if any(num < 0 for num in real_nums):
        print("Has negatives")
    #print(real_nums)
    result = sum(real_nums)
    print(result)
    print(convert_base5_to_snaf(convert_base5_reversed(result)))
    

if __name__ == "__main__":
    main()
    #test()

