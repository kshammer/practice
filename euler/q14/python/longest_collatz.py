import time

attempts = list(range(1000000))[1:]

def main():
    t0 = time.time()
    no_cache_brute()
    t1 = time.time()
    print("No cache brute time {} seconds".format(t1-t0))

    t0 = time.time()
    dict_cache_brute()
    t1 = time.time()
    print("Dict cache brute {}".format(t1-t0))
   
    

def no_cache_brute():
    best_iter = 0 
    best_num = 0
    for num in attempts:
        iterations = collatz(num)
        if iterations > best_iter:
            best_iter = iterations
            best_num = num 
    print("Best number {}, with iterations {}".format(best_num, best_iter))
        

def collatz(num):
    i = 1 
    while(num != 1):
        if num % 2 == 0:
            num = num / 2
            i += 1 
        else: 
            num = 3 * num + 1 
            i += 1 

    return i 

def dict_cache_brute():
    cache = {}
    best_iter = 0
    best_num = 0
    for num in attempts:
        iterations = dict_cache_collatz(num, cache)
        cache[num] = iterations
        if iterations > best_iter:
            best_iter = iterations
            best_num = num 
    print("Best number {}, with iterations {}".format(best_num, best_iter))

def dict_cache_collatz(num, cache):
    i = 1 
    while (num != 1):
        if num in cache.keys():
            return i + cache[num] - 1
        if num % 2 == 0:
            num = num / 2
            i += 1 
        else:
            num = 3 * num + 1 
            i += 1
    return i


if __name__ == "__main__":
    main()
