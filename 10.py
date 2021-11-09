import numpy as np

def six_k(i):
    return 6*i + 1

def is_prime(n):
    """test for primality using trial division
    use 6k +- 1 optimization..
    All primes > 3 take the form 6k +- 1
    So we can test if n is divisible by 2 or 3,
    and if not, test all the way up to 6k+-1 < sqrt(n)
    """
    #do divisability test for 2 and 3 first
    if n <= 3:
        return n > 1 #(2,3 are primes)
    if n % 2 == 0 or n % 3 == 0:
        return False
    count = 0
    i = 5
    print("testing primality of {}".format(n))
    while i ** 2 <= n:
        print("testing with divisors {} and {}".format(i,i+2))
        if n % i == 0 or n % (i + 2) == 0:
            return False
        i += 6
    return True

def prime_sum(n):
    #sum all the primes below n
    #not including 2,3
    sum = 0
    for i in range(n):
        if is_prime(i):
            print("{} is prime.".format(i))
            sum += i
    return sum

if __name__=="__main__":
    print(prime_sum(2000000))
