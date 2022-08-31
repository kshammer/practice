import time

def main():
    start = time.time()
    total = basic()
    end = time.time()
    print("Val {} in time {}".format(total, end-start))



def basic():
    exponent = 1000
    val = 1 << exponent 
    nums = [int(d) for d in str(val)]
    total = sum(nums)
    return total 

if __name__ == "__main__":
    main()
